use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub struct CtorDerive;

impl CtorDerive {
    pub fn derive(input: TokenStream) -> TokenStream {
        let item: ItemStruct =
            syn::parse(input).expect("CtorCall can be only be derived for structs");

        let object_name = item.ident;
        let generics = item.generics;

        let mut parameters = Vec::new();
        let mut field_part = Vec::new();

        for syn::Field { ident, ty, .. } in item.fields {
            parameters.push(quote! {
                #ident: #ty
            });
            field_part.push(quote! { #ident });
        }

        let output = quote! {
            impl #generics #object_name #generics {
                pub fn new(#(#parameters),*) -> Self {
                    Self {
                        #(#field_part),*
                    }
                }
            }
        };

        output.into()
    }
}
