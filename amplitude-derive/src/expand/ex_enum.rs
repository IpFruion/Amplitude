use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Generics, Ident};

pub fn event_enum(ident: Ident, generics: Generics, s: DataEnum) -> syn::Result<TokenStream> {
    let name_match = s
        .variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let ident_to_string = ident.to_string();
            Ok(quote! {
                Self::#ident => #ident_to_string
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        impl amplitude::Event for #ident #generics {
            fn name(&self) -> &str {
                match self {
                    #(#name_match,)*
                }
            }
            fn into_event_props(self) -> Vec<amplitude::event::Property> {
                Vec::new()
            }
        }
    })
}
