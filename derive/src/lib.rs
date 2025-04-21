extern crate proc_macro;

use darling::FromDeriveInput;
use darling::{
    ast::{self, Data, Fields},
    FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, Type};
mod case;
mod serde_parse;

#[proc_macro_derive(TomlInput, attributes(toml_input))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let StructRaw {
        ident,
        attrs,
        data,
        enum_expand,
    } = StructRaw::from_derive_input(&input).unwrap();
    let mut config = Config::default();
    config.enum_expand = enum_expand;
    let schema_token;
    let value_token;
    match data {
        Data::Enum(variants) => {
            schema_token = quote_enum_schema(&ident, &attrs, variants, config);
            value_token = quote_enum_value();
        }
        Data::Struct(fields) => {
            schema_token = quote_struct_schema(&ident, &attrs, fields.clone(), config);
            value_token = quote_struct_value(&attrs, fields.clone());
        }
    }
    let token = quote! {
        impl toml_input::TomlInput for #ident {
            fn schema() -> Result<toml_input::Schema, toml_input::Error> {
                use toml;
                use toml_input::schema;
                #schema_token
            }
            fn into_value(self) -> Result<toml_input::Value, toml_input::Error> {
                #value_token
            }
        }
    };
    // println!("{}", token);
    token.into()
}

fn quote_enum_schema(
    ident: &Ident,
    attrs: &[Attribute],
    variants: Vec<VariantRaw>,
    config: Config,
) -> TokenStream {
    let enum_ident = ident;
    let enum_docs = parse_docs(&attrs);
    let inner_type = enum_ident.to_string();
    let mut tokens = Vec::new();
    for variant in variants {
        let VariantRaw { attrs, enum_expand } = variant;
        let mut config = Config::default();
        config.enum_expand = enum_expand;
        let variant_docs = parse_docs(&attrs);
        let config_token = config.config_token(quote! {variant});
        let variant_token = quote! {
            let mut variant = schema::VariantSchema::default();
            variant.docs = #variant_docs.to_string();
            let value = variant_iter.next().unwrap();
            let tag = std::convert::AsRef::as_ref(&value).to_string();
            let raw = toml::Value::try_from(value).unwrap();
            let prim_value = toml_input::PrimValue {tag, raw: Some(raw)};
            variant.value = prim_value;
            #config_token
            prim_schema.variants.push(variant);
        };
        tokens.push(variant_token);
    }
    let config_token = config.config_token(quote! {meta});
    let enum_token = quote! {
        use strum::IntoEnumIterator;
        let default = <#enum_ident as Default>::default();
        let mut prim_schema = schema::PrimSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = #inner_type.to_string();
        let tag = default.as_ref().to_string();
        let raw = toml::Value::try_from(default).unwrap();
        meta.inner_default = toml_input::PrimValue{tag, raw: Some(raw)};
        meta.defined_docs = #enum_docs.to_string();
        #config_token
        prim_schema.meta = meta;
        let mut variant_iter = #enum_ident::iter();
        prim_schema.variants = Vec::new();
        #(#tokens)*
        Ok(schema::Schema::Prim(prim_schema))
    };
    enum_token
}

fn quote_enum_value() -> TokenStream {
    let enum_token = quote! {
        let tag = self.as_ref().to_string();
        let raw = toml::Value::try_from(self).unwrap();
        let prim = toml_input::PrimValue {tag, raw: Some(raw)};
        Ok(toml_input::Value::Prim(prim))
    };
    enum_token
}

fn quote_struct_schema(
    ident: &Ident,
    attrs: &[Attribute],
    fields: Fields<FieldRaw>,
    config: Config,
) -> TokenStream {
    let struct_ident = ident;
    let struct_docs = parse_docs(&attrs);
    let inner_type = struct_ident.to_string();
    let struct_rename_rule = serde_parse::rename_rule(&attrs);
    let mut tokens = Vec::new();
    for field in fields {
        let FieldRaw {
            ident,
            attrs,
            ty,
            enum_expand,
        } = field;
        let mut config = Config::default();
        config.enum_expand = enum_expand;
        let field_ident = ident.unwrap();
        let field_docs = parse_docs(&attrs);
        let field_rename_rule = serde_parse::rename_rule(&attrs);
        let field_name = field_ident.to_string();
        let field_name = struct_rename_rule.rename_all(&field_name);
        let field_name = field_rename_rule.rename(&field_name);
        let field_flatten = serde_parse::flatten(&attrs);
        let config_token = config.config_token(quote! {field});
        let field_token = quote! {
            let mut field = schema::FieldSchema::default();
            field.ident = #field_name.to_string();
            field.docs = #field_docs.to_string();
            field.flat = #field_flatten;
            field.schema = <#ty as toml_input::TomlInput>::schema()?;
            #config_token
            table.fields.push(field);
        };
        tokens.push(field_token);
    }
    let config_token = config.config_token(quote! {table});
    let struct_token = quote! {
        let default = <#struct_ident as Default>::default();
        let mut table = schema::TableSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = #inner_type.to_string();
        let raw = toml::Value::try_from(default).unwrap();
        meta.inner_default = toml_input::PrimValue::new(raw);
        meta.defined_docs = #struct_docs.to_string();
        table.meta = meta;
        #config_token
        table.fields = Vec::new();
        #(#tokens)*
        Ok(schema::Schema::Table(table))
    };
    struct_token
}

fn quote_struct_value(attrs: &[Attribute], fields: Fields<FieldRaw>) -> TokenStream {
    let struct_rename_rule = serde_parse::rename_rule(&attrs);
    let mut tokens = Vec::new();
    for field in fields {
        let FieldRaw {
            ident,
            attrs,
            enum_expand,
            ..
        } = field;
        let mut config = Config::default();
        config.enum_expand = enum_expand;
        let field_ident = ident.unwrap();
        let field_rename_rule = serde_parse::rename_rule(&attrs);
        let field_name = field_ident.to_string();
        let field_name = struct_rename_rule.rename_all(&field_name);
        let field_name = field_rename_rule.rename(&field_name);
        let field_flatten = serde_parse::flatten(&attrs);
        let field_token = quote! {
            let mut field = toml_input::FieldValue::default();
            field.ident = #field_name.to_string();
            field.flat = #field_flatten;
            field.value = self.#field_ident.into_value()?;
            table.fields.push(field);
        };
        tokens.push(field_token);
    }
    let struct_token = quote! {
        let mut table = toml_input::TableValue::default();
        #(#tokens)*
        Ok(toml_input::Value::Table(table))
    };
    struct_token
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(
    supports(struct_named, enum_any),
    attributes(toml_input),
    forward_attrs(doc, serde)
)]

struct StructRaw {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: ast::Data<VariantRaw, FieldRaw>,
    enum_expand: Option<bool>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct FieldRaw {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    ty: Type,
    enum_expand: Option<bool>,
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct VariantRaw {
    attrs: Vec<Attribute>,
    enum_expand: Option<bool>,
}

fn parse_docs(attrs: &[Attribute]) -> String {
    let mut docs = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let Meta::NameValue(name_value) = &attr.meta {
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) = name_value.value.clone()
            {
                docs.push(lit_str.value());
            }
        }
    }
    docs.join("\n").to_string()
}

#[derive(Clone, Default)]
struct Config {
    enum_expand: Option<bool>,
}

impl Config {
    fn config_token(&self, tag: TokenStream) -> TokenStream {
        let mut token = TokenStream::new();
        if self.enum_expand.is_some() {
            let enum_expand = self.enum_expand;
            token = quote! {
                #token
                #tag.config.enum_expand = #enum_expand;
            }
        }
        token
    }
}
