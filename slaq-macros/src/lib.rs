use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{ItemStruct, Lit, Meta, MetaNameValue, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn slack_api(args: TokenStream, input: TokenStream) -> TokenStream {
    let metas = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
        .parse(args)
        .expect("failed to parse attribute arguments");
    let input_clone = input.clone();
    let item = parse_macro_input!(input as ItemStruct);

    let mut path_lit: Option<String> = None;
    let mut chat_method: Option<syn::Ident> = None;
    let mut response_ty: Option<syn::Ident> = None;
    let mut call_alias: Option<syn::Ident> = None;

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
                ("chat_method", syn::Expr::Path(p)) => {
                    if let Some(id) = p.path.get_ident() {
                        chat_method = Some(id.clone());
                    }
                }
                ("response", syn::Expr::Path(p)) => {
                    if let Some(id) = p.path.get_ident() {
                        response_ty = Some(id.clone());
                    }
                }
                (
                    "call_alias",
                    syn::Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ) => {
                    call_alias = Some(format_ident!("{}", s.value()));
                }
                _ => {}
            }
        }
    }

    let path_lit = path_lit.expect("slack_api requires path=\"...\"");
    let chat_method = chat_method.expect("slack_api requires chat_method=... ident");
    let response_ty = response_ty.expect("slack_api requires response=Type");

    let struct_ident = item.ident.clone();

    // Determine required vs optional fields
    let mut required_fields: Vec<(&syn::Ident, &Type)> = Vec::new();
    let mut optional_fields: Vec<(&syn::Ident, &Type)> = Vec::new();
    for field in &item.fields {
        let ident = field.ident.as_ref().expect("named fields only");
        match is_option(&field.ty) {
            Some(inner) => optional_fields.push((ident, inner)),
            None => required_fields.push((ident, &field.ty)),
        }
    }

    let req_args_new = required_fields.iter().map(|(id, ty)| {
        quote! { #id: impl ::core::convert::Into<#ty> }
    });
    let req_args_chat = required_fields.iter().map(|(id, ty)| {
        quote! { #id: impl ::core::convert::Into<#ty> }
    });
    let req_inits = required_fields.iter().map(|(id, _)| {
        quote! { #id: #id.into() }
    });
    let opt_inits = optional_fields.iter().map(|(id, _)| {
        quote! { #id: ::core::option::Option::None }
    });

    let req_names = required_fields.iter().map(|(id, _)| quote! { #id });

    let opt_setters_req = optional_fields.iter().map(|(id, ty)| {
        if is_bool(ty) {
            quote! {
                #[must_use]
                pub fn #id(mut self, v: bool) -> Self {
                    self.#id = ::core::option::Option::Some(v);
                    self
                }
            }
        } else {
            quote! {
                #[must_use]
                pub fn #id(mut self, v: impl ::core::convert::Into<#ty>) -> Self {
                    self.#id = ::core::option::Option::Some(v.into());
                    self
                }
            }
        }
    });

    // MethodCall impl
    let call_setters = optional_fields.iter().map(|(id, ty)| {
        if is_bool(ty) {
            quote! {
                #[must_use]
                pub fn #id(mut self, v: bool) -> Self {
                    self.inner = self.inner.#id(v);
                    self
                }
            }
        } else {
            quote! {
                #[must_use]
                pub fn #id(mut self, v: impl ::core::convert::Into<#ty>) -> Self {
                    self.inner = self.inner.#id(v);
                    self
                }
            }
        }
    });

    let call_alias_tokens = if let Some(alias) = call_alias {
        quote! { pub type #alias<'a, C> = crate::api::call::MethodCall<'a, C, #struct_ident>; }
    } else {
        quote! {}
    };

    let input_ts: proc_macro2::TokenStream = input_clone.into();
    let expanded = quote! {
        #input_ts

        impl #struct_ident {
            #[must_use]
            pub fn new( #( #req_args_new ),* ) -> Self {
                Self { #( #req_inits ),*, #( #opt_inits ),* }
            }
            #( #opt_setters_req )*
        }

        impl crate::client::SlackMethod for #struct_ident {
            const PATH: &'static str = #path_lit;
            type Body = Self;
            type Response = #response_ty;
            fn into_body(self) -> Self::Body { self }
        }

        impl<'a, C: crate::client::Execute> crate::api::call::MethodCall<'a, C, #struct_ident> {
            #( #call_setters )*
        }

        impl<'a, C: crate::client::Execute> super::Chat<'a, C> {
            pub fn #chat_method(&'a self, #( #req_args_chat ),*) -> crate::api::call::MethodCall<'a, C, #struct_ident> {
                crate::api::call::MethodCall { client: self.client, inner: #struct_ident::new( #( #req_names ),* ) }
            }
        }

        #call_alias_tokens
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
