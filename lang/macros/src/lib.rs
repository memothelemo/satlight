use proc_macro::TokenStream;

mod ctor;
mod utils;

/// Allows to automatically inserts field getters without
/// doing it in a manual labor.
#[proc_macro_derive(FieldCall, attributes(exclude, clone_on_call))]
pub fn derive_field_call(input: TokenStream) -> TokenStream {
    utils::FieldCall::derive(input)
}

#[proc_macro_derive(CtorCall)]
pub fn derive_ctor_node(input: TokenStream) -> TokenStream {
    ctor::CtorDerive::derive(input)
}
