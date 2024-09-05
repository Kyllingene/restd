use proc_macro::{TokenTree, TokenStream, Literal};

#[proc_macro]
pub fn format_args(tokens: TokenStream) -> TokenStream {
    for token in tokens {
        match token {
            TokenTree::Literal(Literal(s)) => todo!(),
            _ => todo!(),
        }
    }

    todo!()
}
