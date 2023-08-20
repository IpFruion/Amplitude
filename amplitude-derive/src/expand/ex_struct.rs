use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields, Generics, Ident};

use crate::attrs::AttrOptions;

use super::prop::event_props;

pub fn event_struct(
    ident: Ident,
    generics: Generics,
    s: DataStruct,
    attrs: AttrOptions,
) -> syn::Result<TokenStream> {
    let ident_to_string = attrs.rename_str(&ident);
    let event_props = match &s.fields {
        Fields::Unit => {
            quote! { Vec::new() }
        }
        Fields::Named(f) => {
            let fields = f
                .named
                .iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let attrs = AttrOptions::parse(&f.attrs)?;
                    Ok((ident, attrs))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            event_props(fields.iter(), true)
        }
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
