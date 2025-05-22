extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn clone_identity(tokens: TokenStream) -> TokenStream {
    tokens.clone()
}

#[proc_macro]
pub fn extend_identity(tokens: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    output.extend(tokens);
    output
}
