use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Generics, Ident};

use super::prop::event_props;

pub fn event_struct(ident: Ident, generics: Generics, s: DataStruct) -> syn::Result<TokenStream> {
    let ident_to_string = ident.to_string();
    let event_props = match &s.fields {
        Fields::Unit => {
            quote! { Vec::new() }
        }
        Fields::Named(_) => event_props(&s.fields)?,
        Fields::Unnamed(_) => Err(syn::Error::new(
            ident.span(),
            "No Amplitude Event property support for unnamed fields",
        ))?,
    };

    Ok(quote! {
        impl amplitude::Event for #ident #generics {
            fn name(&self) -> &str {
                #ident_to_string
            }
            fn into_event_props(self) -> Vec<amplitude::event::Property> {
                #event_props
            }
        }
    })
}
