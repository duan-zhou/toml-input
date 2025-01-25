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
// mod case;

#[proc_macro_derive(TomlSchema)]
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let StructRaw { ident, attrs, data } = StructRaw::from_derive_input(&input).unwrap();
    let token;
    // dbg!(&data);
    match data {
        Data::Enum(variants) => {
            token = quote_enum(&ident, &attrs, variants);
        }
        Data::Struct(fields) => {
            token = quote_struct(&ident, &attrs, fields);
        }
    }
    let token = quote! {
        impl toml_comment::schema::TomlSchema for #ident {
            fn schema() -> toml_comment::schema::Schema {
                use toml_comment::schema;
                use toml_comment::util;
                #token
            }
        }
    };
    token.into()
}

fn quote_enum(ident: &Ident, attrs: &[Attribute], variants: Vec<VariantRaw>) -> TokenStream {
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
        } = variant;
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
        let variant_token = quote! {
            let mut variant = schema::UnitVariant::empty();
            variant.tag = format!("\"{}\"", #variant_name);
            variant.docs = #variant_docs.trim().to_string();
            variant.value = #repr_value;
            root.variants.push(variant);
        };
        tokens.push(variant_token);
    }
    let enum_token = quote! {
        let default = <#enum_ident as Default>::default();
        let mut root = schema::UnitEnum::empty();
        root.wrap_type = "".to_string();
        root.inner_type = #inner_type.to_string();
        root.inner_default = util::value_to_string(&default).unwrap();
        root.docs = #enum_docs.trim().to_string();
        root.variants = Vec::new();
        #(#tokens)*
        schema::Schema::UnitEnum(root)
    };
    enum_token
}

fn quote_struct(ident: &Ident, attrs: &[Attribute], fields: Fields<FieldRaw>) -> TokenStream {
    let struct_ident = ident;
    let struct_docs = parse_docs(&attrs);
    let inner_type = struct_ident.to_string();
    let struct_rename_rule = serde_parse::rename_rule(&attrs);
    let mut tokens = Vec::new();
    for field in fields {
        let FieldRaw { ident, attrs, ty } = field;
        let field_ident = ident.unwrap();
        let field_docs = parse_docs(&attrs);
        let field_rename_rule = serde_parse::rename_rule(&attrs);
        let field_name = field_ident.to_string();
        let field_name = struct_rename_rule.rename_all(&field_name);
        let field_name = field_rename_rule.rename(&field_name);
        let field_flatten = serde_parse::flatten(&attrs);
        let field_token = quote! {
            let mut field = schema::StructField::empty();
            field.ident = #field_name.to_string();
            field.docs = #field_docs.trim().to_string();
            field.flatten = #field_flatten;
            field.schema = <#ty as schema::TomlSchema>::schema();
            root.fields.push(field);
        };
        tokens.push(field_token);
    }
    let struct_token = quote! {
        let default = <#struct_ident as Default>::default();
        let mut root = schema::Struct::empty();
        root.wrap_type = "".to_string();
        root.inner_type = #inner_type.to_string();
        root.inner_default = util::value_to_string(&default).unwrap();
        root.docs = #struct_docs.trim().to_string();
        root.fields = Vec::new();
        #(#tokens)*
        schema::Schema::Struct(root)
    };
    struct_token
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named, enum_unit), forward_attrs(doc, serde))]
struct StructRaw {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: ast::Data<VariantRaw, FieldRaw>,
}

#[derive(Debug, FromField)]
#[darling(forward_attrs(doc, serde))]
struct FieldRaw {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    ty: Type,
}

#[derive(Debug, FromVariant)]
#[darling(forward_attrs(doc, serde))]
struct VariantRaw {
    ident: Ident,
    attrs: Vec<Attribute>,
    discriminant: Option<syn::Expr>,
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
    docs.join("\n").trim_end().to_string()
}
