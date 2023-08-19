use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields};

pub fn event_props(fields: &Fields) -> syn::Result<TokenStream> {
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
