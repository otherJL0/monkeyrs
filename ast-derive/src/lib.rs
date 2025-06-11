extern crate proc_macro;
use proc_macro::TokenStream;
mod node;

#[proc_macro_derive(Node)]
pub fn derive_node_macro(input: TokenStream) -> TokenStream {
    node::derive_proc_macro_impl(input)
}
