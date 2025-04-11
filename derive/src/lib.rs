extern crate proc_macro;

use darling::{
    ast::{self, Data, Fields},
    FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, Type};

use darling::FromDeriveInput;

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
    let token;
    match data {
        Data::Enum(variants) => {
            token = quote_enum(&ident, &attrs, variants, config);
        }
        Data::Struct(fields) => {
            token = quote_struct(&ident, &attrs, fields, config);
        }
    }
    let token = quote! {
        impl toml_input::schema::TomlInput for #ident {
            fn schema() -> toml_input::schema::Schema {
                use toml_input::schema;
                use toml_input::util;
                #token
            }
        }
    };
    token.into()
}

fn quote_enum(
    ident: &Ident,
    attrs: &[Attribute],
    variants: Vec<VariantRaw>,
    config: Config,
) -> TokenStream {
    let enum_ident = ident;
    let enum_docs = parse_docs(&attrs);
    let inner_type = enum_ident.to_string();
    let enum_rename_rule = serde_parse::rename_rule(&attrs);
    let mut tokens = Vec::new();
    let mut last_repr_value = 0;
    for variant in variants {
        let VariantRaw {
            ident: variant_ident,
            attrs,
            discriminant,
            enum_expand,
        } = variant;
        let mut config = Config::default();
        config.enum_expand = enum_expand;
        let repr_value = discriminant
            .map(|token| {
                let s = token.into_token_stream();
                let text = s.to_string().replace(" ", "");
                let value: isize = text.parse().unwrap();
                value
            })
            .unwrap_or(last_repr_value);
        last_repr_value = repr_value + 1;

        let variant_docs = parse_docs(&attrs);
        let variant_rename_rule = serde_parse::rename_rule(&attrs);
        let variant_name = variant_ident.to_string();
        let variant_name = enum_rename_rule.rename_all(&variant_name);
        let variant_name = variant_rename_rule.rename(&variant_name);
        let config_tokens = config.config_tokens(quote! {variant});
        let variant_token = quote! {
            let mut variant = schema::UnitVariant::empty();
            variant.tag = format!("\"{}\"", #variant_name);
            variant.docs = #variant_docs.to_string();
            variant.value = #repr_value;
            #config_tokens
            root.variants.push(variant);
        };
        tokens.push(variant_token);
    }
    let config_tokens = config.config_tokens(quote! {root});
    let enum_token = quote! {
        let default = <#enum_ident as Default>::default();
        let mut root = schema::UnitEnum::empty();
        root.wrap_type = "".to_string();
        root.inner_type = #inner_type.to_string();
        root.inner_default = util::value_to_string(&default).unwrap();
        root.docs = #enum_docs.to_string();
        #config_tokens
        root.variants = Vec::new();
        #(#tokens)*
        schema::Schema::UnitEnum(root)
    };
    enum_token
}

fn quote_struct(
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
        let config_tokens = config.config_tokens(quote! {field});
        let field_token = quote! {
            let mut field = schema::StructField::empty();
            field.ident = #field_name.to_string();
            field.docs = #field_docs.to_string();
            field.flatten = #field_flatten;
            field.schema = <#ty as schema::TomlInput>::schema();
            #config_tokens
            root.fields.push(field);
        };
        tokens.push(field_token);
    }
    let config_tokens = config.config_tokens(quote! {root});
    let struct_token = quote! {
        let default = <#struct_ident as Default>::default();
        let mut root = schema::Struct::empty();
        root.wrap_type = "".to_string();
        root.inner_type = #inner_type.to_string();
        root.inner_default = util::value_to_string(&default).unwrap();
        root.docs = #struct_docs.to_string();
        #config_tokens
        root.fields = Vec::new();
        #(#tokens)*
        schema::Schema::Struct(root)
    };
    struct_token
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    supports(struct_named, enum_unit, enum_tuple),
    attributes(toml_input),
    forward_attrs(doc, serde)
)]
struct StructRaw {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: ast::Data<VariantRaw, FieldRaw>,
    enum_expand: Option<bool>,
}

#[derive(Debug, FromField)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct FieldRaw {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    ty: Type,
    enum_expand: Option<bool>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(toml_input), forward_attrs(doc, serde))]
struct VariantRaw {
    ident: Ident,
    attrs: Vec<Attribute>,
    discriminant: Option<syn::Expr>,
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
    fn config_tokens(&self, tag: TokenStream) -> TokenStream {
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
