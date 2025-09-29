use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Attribute, ItemStruct, Lit, Meta, MetaNameValue, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn slack_api(args: TokenStream, input: TokenStream) -> TokenStream {
    let metas = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
        .parse(args)
        .expect("failed to parse attribute arguments");
    let item = parse_macro_input!(input as ItemStruct);

    let mut path_lit: Option<String> = None;
    let mut response_ty: Option<syn::Ident> = None;

    for meta in metas {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta
            && let Some(ident) = path.get_ident().cloned()
        {
            let key = ident.to_string();
            match (key.as_str(), value) {
                (
                    "path",
                    syn::Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ) => {
                    path_lit = Some(s.value());
                }
                ("response", syn::Expr::Path(p)) => {
                    if let Some(id) = p.path.get_ident() {
                        response_ty = Some(id.clone());
                    }
                }
                _ => {}
            }
        }
    }

    let path_lit = path_lit.expect("slack_api requires path=\"...\"");
    let response_ty = response_ty.expect("slack_api requires response=Type");

    let struct_ident = item.ident.clone();

    // Determine required vs optional fields
    let mut required_fields: Vec<(&syn::Ident, &Type)> = Vec::new();
    let mut optional_fields: Vec<(&syn::Ident, &Type, Vec<Attribute>)> = Vec::new();
    for field in &item.fields {
        let ident = field.ident.as_ref().expect("named fields only");
        match is_option(&field.ty) {
            Some(inner) => {
                let docs: Vec<Attribute> = field
                    .attrs
                    .iter()
                    .filter(|a| a.path().is_ident("doc"))
                    .cloned()
                    .collect();
                optional_fields.push((ident, inner, docs));
            }
            None => required_fields.push((ident, &field.ty)),
        }
    }

    let req_args_new = required_fields.iter().map(|(id, ty)| {
        quote! { #id: impl ::core::convert::Into<#ty> }
    });
    let req_inits = required_fields.iter().map(|(id, _)| {
        quote! { #id: #id.into() }
    });
    let opt_inits = optional_fields.iter().map(|(id, _, _)| {
        quote! { #id: ::core::option::Option::None }
    });
    let field_inits: Vec<proc_macro2::TokenStream> = req_inits.chain(opt_inits).collect();

    let opt_setters = optional_fields.iter().map(|(id, ty, docs)| {
        if is_bool(ty) {
            quote! {
                #( #docs )*
                #[must_use]
                pub fn #id(mut self, v: bool) -> Self {
                    self.#id = ::core::option::Option::Some(v);
                    self
                }
            }
        } else {
            quote! {
                #( #docs )*
                #[must_use]
                pub fn #id(mut self, v: impl ::core::convert::Into<#ty>) -> Self {
                    self.#id = ::core::option::Option::Some(v.into());
                    self
                }
            }
        }
    });

    let chat_path_doc = format!("Slack API path: {path_lit}");
    let expanded = quote! {
        #item

        impl #struct_ident {
            #[must_use]
            pub fn new( #( #req_args_new ),* ) -> Self {
                Self { #( #field_inits ),* }
            }
            #( #opt_setters )*
            /// Builds a transport-agnostic Slack request containing this payload.
            #[must_use]
            #[doc = #chat_path_doc]
            pub fn build_request(self) -> crate::client::SlackRequest<Self> {
                crate::client::SlackRequest::from(self)
            }
        }

        impl crate::client::SlackMethod for #struct_ident {
            const PATH: &'static str = #path_lit;
            type Body = Self;
            type Response = #response_ty;
            fn into_body(self) -> Self::Body { self }
        }
    };

    expanded.into()
}

fn is_option(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
        && seg.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return Some(inner);
    }
    None
}

fn is_bool(ty: &Type) -> bool {
    if let Type::Path(tp) = ty
        && let Some(seg) = tp.path.segments.last()
    {
        return seg.ident == "bool" && seg.arguments.is_empty();
    }
    false
}

#[proc_macro_attribute]
pub fn block(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse attribute args like: kind = "divider"
    let metas = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
        .parse(args)
        .expect("failed to parse attribute arguments");

    let mut kind_lit: Option<String> = None;
    for meta in metas {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta
            && let Some(ident) = path.get_ident().cloned()
        {
            let key = ident.to_string();
            if let (
                "kind",
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Str(s), ..
                }),
            ) = (key.as_str(), value)
            {
                kind_lit = Some(s.value());
            }
        }
    }

    let kind_lit = kind_lit.expect("block requires kind=\"...\"");

    let item = parse_macro_input!(input as ItemStruct);
    let struct_ident = item.ident.clone();

    // Determine required vs optional fields
    let mut required_fields: Vec<(&syn::Ident, &Type)> = Vec::new();
    let mut optional_fields: Vec<(&syn::Ident, &Type, Vec<Attribute>)> = Vec::new();
    for field in &item.fields {
        let ident = field.ident.as_ref().expect("named fields only");
        match is_option(&field.ty) {
            Some(inner) => {
                let docs: Vec<Attribute> = field
                    .attrs
                    .iter()
                    .filter(|a| a.path().is_ident("doc"))
                    .cloned()
                    .collect();
                optional_fields.push((ident, inner, docs));
            }
            None => required_fields.push((ident, &field.ty)),
        }
    }

    // new(required...)
    let req_args_new = required_fields.iter().map(|(id, ty)| {
        quote! { #id: impl ::core::convert::Into<#ty> }
    });
    let req_inits = required_fields.iter().map(|(id, _)| {
        quote! { #id: #id.into() }
    });
    let opt_inits = optional_fields.iter().map(|(id, _, _)| {
        quote! { #id: ::core::option::Option::None }
    });
    let field_inits: Vec<proc_macro2::TokenStream> = req_inits.chain(opt_inits).collect();

    // optional setters
    let opt_setters = optional_fields.iter().map(|(id, ty, docs)| {
        if is_bool(ty) {
            quote! {
                #( #docs )*
                #[must_use]
                pub fn #id(mut self, v: bool) -> Self {
                    self.#id = ::core::option::Option::Some(v);
                    self
                }
            }
        } else {
            quote! {
                #( #docs )*
                #[must_use]
                pub fn #id(mut self, v: impl ::core::convert::Into<#ty>) -> Self {
                    self.#id = ::core::option::Option::Some(v.into());
                    self
                }
            }
        }
    });

    // build() -> crate::blocks::Block
    let mut build_inserts_req: Vec<proc_macro2::TokenStream> = Vec::new();
    for (id, _) in &required_fields {
        let key = id.to_string();
        build_inserts_req.push(quote! {
            map.insert(
                ::std::string::String::from(#key),
                ::serde_json::to_value(self.#id).expect("serialize required field"),
            );
        });
    }
    let mut build_inserts_opt: Vec<proc_macro2::TokenStream> = Vec::new();
    for (id, _, _) in &optional_fields {
        let key = id.to_string();
        build_inserts_opt.push(quote! {
            if let ::core::option::Option::Some(v) = self.#id {
                map.insert(
                    ::std::string::String::from(#key),
                    ::serde_json::to_value(v).expect("serialize optional field"),
                );
            }
        });
    }

    let expanded = {
        quote! {
            #item

            impl #struct_ident {
                #[must_use]
                pub fn new( #( #req_args_new ),* ) -> Self {
                    Self { #( #field_inits ),* }
                }
                #( #opt_setters )*
                #[must_use]
                pub fn build(self) -> crate::blocks::Block {
                    let mut map = ::serde_json::Map::new();
                    map.insert(
                        ::std::string::String::from("type"),
                        ::serde_json::Value::String(::std::string::String::from(#kind_lit)),
                    );
                    #( #build_inserts_req )*
                    #( #build_inserts_opt )*
                    crate::blocks::Block(::serde_json::Value::Object(map))
                }
            }
        }
    };

    expanded.into()
}
