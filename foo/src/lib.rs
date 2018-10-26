extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro]
pub fn bar(_input: TokenStream) -> TokenStream {
    let out = quote! {
        const BAR: &str = "bar";
    };
    out.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
