use heck::ToTitleCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn value(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let attrs = ast.attrs;
    let name = ast.ident;
    let name_mut = Ident::new(&format!("{name}Mut"), name.span());

    let variants: Vec<_> = args
        .to_string()
        .replace(' ', "")
        .replace('\n', "")
        .replace('\r', "")
        .split(',')
        .map(|ty| {
            let var_name = ty.to_title_case();
            let var_name = Ident::new(&var_name, Span::call_site());
            let ty = Ident::new(ty, Span::call_site());
            (var_name, ty)
        })
        .collect();

    let getters: Vec<_> = variants
        .iter()
        .map(|(var_name, ty)| {
            let fn_name = Ident::new(&format!("get_{ty}"), Span::call_site());
            quote! {
                pub fn #fn_name(&self) -> Option<&#ty> {
                    match self {
                        Self::#var_name(x) => Some(x),
                        _ => None,
                    }
                }
            }
        })
        .collect();

    let getters_mut: Vec<_> = variants
        .iter()
        .map(|(var_name, ty)| {
            let fn_name = Ident::new(&format!("get_{ty}_mut"), Span::call_site());
            quote! {
                pub fn #fn_name(&mut self) -> Option<&mut #ty> {
                    match self {
                        Self::#var_name(x) => Some(x),
                        _ => None,
                    }
                }
            }
        })
        .collect();

    let sindex_impl: Vec<_> = variants
        .iter()
        .map(|(var_name, ty)| {
            quote! {
                impl Sindex for #ty {
                    fn sindex<'a>(&'a self, key: &str) -> Option<Value<'a>> {
                        if key.is_empty() {
                            Some(Value::#var_name(self))
                        } else {
                            None
                        }
                    }

                    fn sindex_mut<'a>(&'a mut self, key: &str) -> Option<ValueMut<'a>> {
                        if key.is_empty() {
                            Some(ValueMut::#var_name(self))
                        } else {
                            None
                        }
                    }
                }
            }
        })
        .collect();

    let setters: Vec<_> = variants
        .iter()
        .filter(|(_, ty)| ty != "str")
        .map(|(var_name, ty)| {
            let fn_name = Ident::new(&format!("set_{ty}"), Span::call_site());
            quote! {
                pub fn #fn_name(&mut self, value: #ty) -> Option<#ty> {
                    match self {
                        Self::#var_name(x) => {
                            let old_value = **x;
                            **x = value;
                            Some(old_value)
                        },
                        _ => None,
                    }
                }
            }
        })
        .collect();

    let out_variants: Vec<_> = variants
        .iter()
        .map(|(var_name, ty)| {
            quote! {#var_name(&'a #ty)}
        })
        .collect();

    let out_variants_mut: Vec<_> = variants
        .iter()
        .map(|(var_name, ty)| {
            quote! {#var_name(&'a mut #ty)}
        })
        .collect();

    let output = quote! {
        #(#attrs)*
        pub enum #name<'a> {
            #(#out_variants,)*
        }

        pub enum #name_mut<'a> {
            #(#out_variants_mut,)*
        }

        impl<'a> #name<'a> {
            #(#getters)*
        }

        impl<'a> #name_mut<'a> {
            #(#getters)*
            #(#getters_mut)*
            #(#setters)*
        }

        #(#sindex_impl)*
    };
    // println!("{output}");
    output.into()
}
