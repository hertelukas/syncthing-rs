use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let builder_ident = syn::Ident::new(&format!("New{}", ident), ident.span());

    let expanded = quote! {
        pub struct #builder_ident {

        }

        impl #builder_ident {

        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/01-parse.rs");
    }
}
