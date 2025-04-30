extern crate proc_macro;

use darling::{
    ast::{self, Data, Fields},
    FromField, FromVariant,
};
use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, PathArguments,
    Type, TypePath,
};
mod serde_parse;

#[proc_macro_derive(TomlInput, attributes(toml_input))]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let StructRaw {
        ident,
        attrs,
        data,
        enum_style,
    } = StructRaw::from_derive_input(&input).unwrap();
    let config = Config {
        enum_style,
        ..Default::default()
    };
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
                use toml_input::config::EnumStyle;
                #schema_token
            }
            fn into_value(self) -> Result<toml_input::Value, toml_input::Error> {
                #value_token
            }
        }
    };
    token.into()
}

fn quote_enum_schema(
    ident: &Ident,
    attrs: &[Attribute],
    variants: Vec<VariantRaw>,
    config: Config,
) -> TokenStream {
    let enum_ident = ident;
    let enum_docs = parse_docs(attrs);
    let inner_type = enum_ident.to_string();
    let mut tokens = Vec::new();
    for variant in variants {
        let VariantRaw { attrs, enum_style } = variant;
        let variant_docs = parse_docs(&attrs);
        let variant_config = Config {
            enum_style: enum_style.or(config.enum_style.clone()),
            ..Default::default()
        };
        let enum_style_token = variant_config.enum_style_token(quote! {variant});
        let variant_token = quote! {
            let mut variant = schema::VariantSchema::default();
            variant.docs = #variant_docs.to_string();
            let value = variant_iter.next().ok_or(toml_input::Error::EnumEmpty)?;
            let tag = std::convert::AsRef::as_ref(&value).to_string();
            let raw = toml::Value::try_from(value)?;
            let prim_value = toml_input::PrimValue {tag, raw: Some(raw)};
            variant.value = prim_value;
            #enum_style_token
            prim_schema.variants.push(variant);
        };
        tokens.push(variant_token);
    }
    let enum_style_token = config.enum_style_token(quote! {meta});
    let enum_token = quote! {
        use strum::IntoEnumIterator;
        let default = <#enum_ident as Default>::default();
        let mut prim_schema = schema::PrimSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = #inner_type.to_string();
        let tag = default.as_ref().to_string();
        let raw = toml::Value::try_from(default)?;
        meta.inner_default = toml_input::PrimValue{tag, raw: Some(raw)};
        meta.defined_docs = #enum_docs.to_string();
        #enum_style_token;
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
        let raw = toml::Value::try_from(self)?;
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
    let struct_docs = parse_docs(attrs);
    let inner_type = struct_ident.to_string();
    let struct_rule = serde_parse::rename_rule(attrs);
    let mut tokens = Vec::new();
    for field in fields {
        let FieldRaw {
            ident,
            attrs,
            ty,
            enum_style,
            inner_default,
        } = field;
        let field_ident = ident.unwrap();
        let field_docs = parse_docs(&attrs);
        let field_rule = serde_parse::rename_rule(&attrs);
        let field_name = field_ident.to_string();
        let field_name = struct_rule.case_to(field_name);
        let field_name = field_rule.alias(field_name);
        let field_flatten = serde_parse::flatten(&attrs);
        let field_config = Config {
            enum_style: enum_style.or(config.enum_style.clone()),
            inner_default,
        };
        let enum_style_token = field_config.enum_style_token(quote! {field});
        let inner_type = extract_inner_type(&ty);
        let inner_default_token = field_config.inner_default_token(quote! {field}, inner_type);
        let field_token = quote! {
            let mut field = schema::FieldSchema::default();
            field.ident = #field_name.to_string();
            field.docs = #field_docs.to_string();
            field.flat = #field_flatten;
            field.schema = <#ty as toml_input::TomlInput>::schema()?;
            #enum_style_token
            #inner_default_token
            table.fields.push(field);
        };
        tokens.push(field_token);
    }
    let enum_style_token = config.enum_style_token(quote! {table});
    let struct_token = quote! {
        use std::str::FromStr;
        let default = <#struct_ident as Default>::default();
        let mut table = schema::TableSchema::default();
        let mut meta = schema::Meta::default();
        meta.wrap_type = "".to_string();
        meta.inner_type = #inner_type.to_string();
        let raw = toml::Value::try_from(default)?;
        meta.inner_default = toml_input::PrimValue::new(raw);
        meta.defined_docs = #struct_docs.to_string();
        table.meta = meta;
        #enum_style_token
        table.fields = Vec::new();
        #(#tokens)*
        Ok(schema::Schema::Table(table))
    };
    struct_token
}

fn quote_struct_value(attrs: &[Attribute], fields: Fields<FieldRaw>) -> TokenStream {
    let struct_rule = serde_parse::rename_rule(attrs);
    let mut tokens = Vec::new();
    for field in fields {
        let FieldRaw { ident, attrs, .. } = field;
        let field_ident = ident.unwrap();
        let field_rule = serde_parse::rename_rule(&attrs);
        let field_name = field_ident.to_string();
        let field_name = struct_rule.case_to(field_name);
        let field_name = field_rule.alias(field_name);
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
    enum_style: Option<EnumStyle>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct FieldRaw {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    ty: Type,
    enum_style: Option<EnumStyle>,
    inner_default: Option<String>,
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct VariantRaw {
    attrs: Vec<Attribute>,
    enum_style: Option<EnumStyle>,
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

fn extract_inner_type(ty: &syn::Type) -> TokenStream {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                    return inner_ty.into_token_stream();
                }
            }
        }
    }
    TokenStream::new()
}

#[derive(Clone, Default)]
struct Config {
    enum_style: Option<EnumStyle>,
    inner_default: Option<String>,
}

impl Config {
    fn enum_style_token(&self, tag: TokenStream) -> TokenStream {
        let mut token = TokenStream::new();
        if let Some(enum_style) = &self.enum_style {
            token = quote! {
                #tag.config.enum_style = Some(#enum_style);
            };
        }
        token
    }

    fn inner_default_token(&self, tag: TokenStream, inner_type: TokenStream) -> TokenStream {
        let mut token = TokenStream::new();
        if inner_type.is_empty() {
            return token;
        }
        if let Some(text) = &self.inner_default {
            token = quote! {
                let value = #inner_type::from_str(#text).map_err(|err| toml_input::Error::FromStrError(err.to_string()))?;
                let raw = toml::Value::try_from(value)?;
                #tag.set_inner_default(raw);
            };
        }
        token
    }
}

#[derive(Debug, Clone, FromMeta)]
#[derive(Default)]
enum EnumStyle {
    Single,
    #[default]
    Expand,
    Fold,
    Flex,
    Flex4,
    Flex5,
    Flex6,
    Flex7,
    Flex8,
    Flex9,
    Flex10,
    Flex11,
    Flex12,
}


impl ToTokens for EnumStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = match self {
            EnumStyle::Single => quote! { EnumStyle::Single },
            EnumStyle::Expand => quote! { EnumStyle::Expand },
            EnumStyle::Fold => quote! { EnumStyle::Fold },
            EnumStyle::Flex => quote! { EnumStyle::Flex(4) },
            EnumStyle::Flex4 => quote! { EnumStyle::Flex(4) },
            EnumStyle::Flex5 => quote! { EnumStyle::Flex(5) },
            EnumStyle::Flex6 => quote! { EnumStyle::Flex(6) },
            EnumStyle::Flex7 => quote! { EnumStyle::Flex(7) },
            EnumStyle::Flex8 => quote! { EnumStyle::Flex(8) },
            EnumStyle::Flex9 => quote! { EnumStyle::Flex(9) },
            EnumStyle::Flex10 => quote! { EnumStyle::Flex(10) },
            EnumStyle::Flex11 => quote! { EnumStyle::Flex(11) },
            EnumStyle::Flex12 => quote! { EnumStyle::Flex(12) },
        };
        tokens.extend(token);
    }
}
