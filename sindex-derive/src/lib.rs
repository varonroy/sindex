use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Sindex)]
pub fn sindex(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let struct_data = match &ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let matches = struct_data.fields.iter().map(|field| {
        let ident = field.ident.clone();
        let field_name = field.ident.clone().unwrap().to_string();
        quote! {#field_name => self.#ident.sindex(remaining_key)}
    });

    let matches_mut = struct_data.fields.iter().map(|field| {
        let ident = field.ident.clone();
        let field_name = field.ident.clone().unwrap().to_string();
        quote! {#field_name => self.#ident.sindex_mut(remaining_key)}
    });

    let output = quote! {
        impl sindex::Sindex for #name {
            fn sindex(&self, key: &str) -> Option<sindex::Value> {
                let (current_key, remaining_key) = if let Some(i) = key.find(".") {
                    (&key[..i], &key[i + 1..])
                } else {
                    (key, "")
                };

                match current_key {
                    #(#matches,)*
                    _ => None,
                }
            }

            fn sindex_mut(&mut self, key: &str) -> Option<sindex::ValueMut> {
                let (current_key, remaining_key) = if let Some(i) = key.find(".") {
                    (&key[..i], &key[i + 1..])
                } else {
                    (key, "")
                };

                match current_key {
                    #(#matches_mut,)*
                    _ => None,
                }
            }
        }
    };

    output.into()
}
