use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Field, Fields, Generics, Ident};

pub fn event_derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = input.ident;
    let generics = input.generics;
    match input.data {
        syn::Data::Struct(d) => event_struct(ident, generics, d),
        _ => Err(syn::Error::new(
            ident.span(),
            "Only available for structs (at the moment)",
        )),
    }
}

fn event_struct(ident: Ident, generics: Generics, s: DataStruct) -> syn::Result<TokenStream> {
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

fn event_props(fields: &Fields) -> syn::Result<TokenStream> {
    let expanded_props = fields
        .iter()
        .map(event_prop)
        .collect::<syn::Result<Vec<_>>>()?;

    let expanded = quote! {
        vec![
            #(#expanded_props,)*
        ]
    };

    Ok(expanded.into())
}

fn event_prop(field: &Field) -> syn::Result<TokenStream> {
    // The expectation is that this is Named
    let ident = field.ident.as_ref().unwrap();
    let ident_to_string = ident.to_string();
    Ok(quote! {amplitude::event::Property {
        name: #ident_to_string.to_owned(),
        value: self.#ident.into(),
    }})
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Type};

    #[test]
    pub fn create_base() {
        let ty: Type = parse_quote!(u8);
        println!("{:?}", ty);
    }
}
