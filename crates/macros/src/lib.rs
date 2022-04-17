extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PropertyGetter)]
pub fn call_each_property(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    match input.data {
        syn::Data::Struct(structure) => {
            let (fields, param_types, struct_decl) = {
                let mut methods = Vec::new();
                let mut declarations = Vec::new();
                let mut decl = Vec::new();
                for field in structure.fields.iter() {
                    let ident = field.ident.as_ref().unwrap();
                    let ret_ty = &field.ty;

                    decl.push(quote! {
                        #ident
                    });

                    declarations.push(quote! {
                        #ident: #ret_ty
                    });

                    if let syn::Visibility::Public(_) = &field.vis {
                        continue;
                    }

                    // a bug fix, sort of?
                    if *ident == "span" {
                        continue;
                    }

                    methods.push(quote! {
                        pub fn #ident(&self) -> &#ret_ty {
                            &self.#ident
                        }
                    });
                }
                (methods, declarations, decl)
            };

            let generics = input.generics;
            let expanded = quote! {
                impl #generics #name #generics {
                    #(#fields)*

                    pub fn new(#(#param_types),*) -> Self {
                        #name {
                            #(#struct_decl),*
                        }
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Unions and enums are not supported, implement it manually."),
    }
}

#[proc_macro_derive(HirNode)]
pub fn hir_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(structure) => {
            let mut has_original = false;
            for field in structure.fields.iter() {
                if let Some(ident) = &field.ident {
                    if *ident == "original" {
                        has_original = true;
                        break;
                    }
                }
            }

            if !has_original {
                panic!("Consider adding `original` field member or implement HirNode manually instead.");
            }

            let generics = input.generics;
            let expanded = quote! {
                impl #generics HirNode for #name #generics {
                    fn span(&self) -> Span {
                        use lunar_shared::Node;
                        self.original.span()
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Unions and enums are not supported, implement it manually."),
    }
}
