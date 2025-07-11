use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn is_required(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("required"))
}

fn get_rename(attrs: &[syn::Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        // Only interested in attributes that look like #[serde(...)]
        if attr.path().is_ident("serde") {
            if let syn::Attribute {
                meta: syn::Meta::List(syn::MetaList { tokens, .. }),
                ..
            } = attr
            {
                let tokens: Vec<proc_macro2::TokenTree> = tokens.clone().into_iter().collect();
                if let Some(proc_macro2::TokenTree::Ident(ident)) = tokens.first() {
                    if *ident == "rename" {
                        if let Some(proc_macro2::TokenTree::Literal(name)) = tokens.get(2) {
                            // Convert the literal into a string like "\"bar\""
                            let value = name.to_string();

                            // Parse that string back into a syn::Lit (which handles quotes properly)
                            if let Ok(lit) = syn::parse_str::<syn::LitStr>(&value) {
                                return Some(quote! { #[serde(rename = #lit)] });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_rename_all(attrs: &[syn::Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        // Only interested in attributes that look like #[serde(...)]
        if attr.path().is_ident("serde") {
            if let syn::Attribute {
                meta: syn::Meta::List(syn::MetaList { tokens, .. }),
                ..
            } = attr
            {
                let tokens: Vec<proc_macro2::TokenTree> = tokens.clone().into_iter().collect();
                if let Some(proc_macro2::TokenTree::Ident(ident)) = tokens.first() {
                    if *ident == "rename_all" {
                        if let Some(proc_macro2::TokenTree::Literal(name)) = tokens.get(2) {
                            // Convert the literal into a string like "\"bar\""
                            let value = name.to_string();

                            // Parse that string back into a syn::Lit (which handles quotes properly)
                            if let Ok(lit) = syn::parse_str::<syn::LitStr>(&value) {
                                return Some(quote! { #[serde(rename_all = #lit)] });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[proc_macro_derive(New, attributes(required))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let builder_ident = syn::Ident::new(&format!("New{ident}"), ident.span());

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
        let rename = get_rename(&field.attrs);
        if !is_required(&field.attrs) {
            quote! {
            #rename
            #[serde(skip_serializing_if = "Option::is_none")]
            #name: std::option::Option<#ty>}
        } else {
            quote! {
                #rename
                #name: #ty
            }
        }
    });

    let funcs = fields.clone().map(|field| {
        let name = &field.ident;
        if let Some(name) = name {
            let getter = format!("get_{name}");
            let getter = proc_macro2::Ident::new(&getter, name.span());
            let ty = &field.ty;
            if !is_required(&field.attrs) {
                quote! {
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.#name = std::option::Option::Some(#name);
                        self
                    }
                        pub fn #getter(&self) -> &std::option::Option<#ty> {
                            &self.#name
                        }
                }
            } else {
                quote! {
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.#name = #name;
                        self
                    }

                    pub fn #getter(&self) -> &#ty {
                        &self.#name
                    }
                }
            }
        } else {
            // TODO raise error
            quote! {}
        }
    });

    let from = fields.clone().map(|field| {
        let name = &field.ident;
        if !is_required(&field.attrs) {
            quote! { #name: Some(value.#name) }
        } else {
            quote! { #name: value.#name }
        }
    });

    let rename_all = get_rename_all(&input.attrs);

    let expanded = quote! {
        #[derive(Clone, Debug, PartialEq, serde::Serialize)]
        #rename_all
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

        impl From<#ident> for #builder_ident {
            fn from(value: #ident) -> Self {
                Self {
                    #(#from),*
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
        t.pass("tests/ui/*.rs");
    }
}
