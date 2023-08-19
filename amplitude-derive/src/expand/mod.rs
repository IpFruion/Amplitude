use proc_macro2::TokenStream;
use syn::DeriveInput;

use self::{ex_enum::event_enum, ex_struct::event_struct};

mod ex_enum;
mod ex_struct;
mod prop;

pub fn event_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = input.ident;
    let generics = input.generics;
    match input.data {
        syn::Data::Struct(d) => event_struct(ident, generics, d),
        syn::Data::Enum(e) => event_enum(ident, generics, e),
        _ => Err(syn::Error::new(
            ident.span(),
            "Only available for structs (at the moment)",
        )),
    }
}
