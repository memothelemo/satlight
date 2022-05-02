use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Visibility};

pub struct FieldCall;

impl FieldCall {
    pub fn derive(input: TokenStream) -> TokenStream {
        let item: ItemStruct =
            syn::parse(input).expect("FieldCall can be only be derived for structs");

        let object_name = item.ident;
        let generics = item.generics;

        let mut methods = Vec::new();

        for field in item.fields {
            let mut is_excluded = false;
            let mut no_ref = false;
            //let mut attributes = Vec::new();
            for attr in field.attrs {
                let attr_name = match attr.path.get_ident() {
                    Some(ident) => ident.to_string(),
                    None => continue,
                };
                match attr_name.as_str() {
                    "exclude" => is_excluded = true,
                    "clone_on_call" => no_ref = true,
                    _ => {}
                };
            }
            if !matches!(field.vis, Visibility::Public(..)) && !is_excluded {
                //&& field.ident.is_some() {
                let identifier = field.ident.unwrap();
                let ty = field.ty;
                let ty = if no_ref {
                    quote! { #ty }
                } else {
                    quote! { &#ty }
                };
                let call_clone = if no_ref {
                    quote! { .clone() }
                } else {
                    quote! {}
                };
                let self_ = if no_ref {
                    quote! {self}
                } else {
                    quote! {&self}
                };
                methods.push(quote! {
                    #[allow(missing_docs)]
                    pub fn #identifier(&self) -> #ty {
                        #self_.#identifier#call_clone
                    }
                });
            }
        }

        let output = quote! {
            impl #generics #object_name #generics {
                #(#methods)*
            }
        };

        output.into()
    }
}
