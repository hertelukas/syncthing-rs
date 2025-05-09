use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn is_required(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("required"))
}

#[proc_macro_derive(New, attributes(required))]
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

    let new_fields = fields.clone().map(|field| {
        let name = &field.ident;
        if !is_required(&field.attrs) {
            quote! {#name: std::option::Option::None}
        } else {
            quote! {#name}
        }
    });

    let required_args = fields.clone().filter_map(|field| {
        if is_required(&field.attrs) {
            let name = &field.ident;
            let ty = &field.ty;
            Some(quote! {#name: #ty})
        } else {
            None
        }
    });

    let options = fields.clone().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        if !is_required(&field.attrs) {
            quote! { #name: std::option::Option<#ty>}
        } else {
            quote! {#name: #ty}
        }
    });

    let funcs = fields.clone().map(|field| {
        let name = &field.ident;

        let ty = &field.ty;
        // TODO not sure if thats ideal -> we will only have references
        // to the object if chained.
        if !is_required(&field.attrs) {
            quote! {fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = std::option::Option::Some(#name);
                self
            }}
        } else {
            quote! {fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = #name;
                self
            }}
        }
    });

    let expanded = quote! {
        #[derive(serde::Serialize)]
        pub struct #builder_ident {
            #(#options),*
        }

        impl #builder_ident {
            pub fn new(#(#required_args),*) -> Self {
                Self {
                    #(#new_fields),*
                }
            }

            #(#funcs)*
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
        t.pass("tests/ui/03-setter.rs");
        t.pass("tests/ui/04-chained.rs");
        t.pass("tests/ui/05-required.rs");
    }
}
