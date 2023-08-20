use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn event_props<'a, I: Iterator<Item = &'a Ident>>(
    fields: I,
    contain_self: bool,
) -> TokenStream {
    let expanded_props = fields.map(|i| event_prop(i, contain_self));
    quote! {
        vec![
            #(#expanded_props,)*
        ]
    }
}

fn event_prop(ident: &Ident, contain_self: bool) -> TokenStream {
    let ident_to_string = ident.to_string();
    let value = if contain_self {
        quote!(self.#ident)
    } else {
        quote!(#ident)
    };
    quote! {amplitude::event::Property {
        name: #ident_to_string.to_owned(),
        value: #value.into(),
    }}
}

pub fn into_event_props<'a, I: Iterator<Item = &'a Ident>>(mut fields: I) -> TokenStream {
    // Must be at least one I think handle error?
    let first = fields.next().unwrap();
    let props = quote! {
        {
            let mut props = amplitude::Event::into_event_props(#first);
            props
        }
    };

    props
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
