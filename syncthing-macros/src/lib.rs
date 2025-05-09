use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let builder_ident = syn::Ident::new(&format!("New{}", ident), ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named.iter()
    } else {
        unimplemented!()
    };

    let none_fields = fields.clone().map(|field| {
        let name = &field.ident;
        quote! {#name: None}
    });

    let options = fields.clone().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { #name: std::option::Option<#ty>}
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#options),*
        }

        impl #builder_ident {
            pub fn new() -> Self {
                Self {
                    #(#none_fields),*
                }
            }
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
        t.pass("tests/ui/02-create-new.rs");
    }
}
