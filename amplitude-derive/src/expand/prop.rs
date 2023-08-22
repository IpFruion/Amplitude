use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident, Type};

use crate::attrs::AttrOptions;

pub struct FieldProp<'a> {
    pub ident: &'a Ident,
    pub ty: &'a Type,
    pub attrs: AttrOptions,
}

impl<'a> TryFrom<&'a Field> for FieldProp<'a> {
    type Error = syn::Error;
    fn try_from(value: &'a Field) -> Result<Self, Self::Error> {
        Ok(Self {
            ident: value.ident.as_ref().unwrap(),
            ty: &value.ty,
            attrs: AttrOptions::parse(&value.attrs)?,
        })
    }
}

pub fn event_props<'a, I: Iterator<Item = &'a FieldProp<'a>>>(
    fields: I,
    contain_self: bool,
) -> TokenStream {
    let expanded_props = fields.map(|p| event_prop(p, contain_self));
    quote! {{
        let mut props = Vec::new();
        #(#expanded_props;)*
        props
    }}
}

fn event_prop<'a>(p: &FieldProp<'a>, contain_self: bool) -> TokenStream {
    let is_option = match p.ty {
        Type::Path(path) => path
            .path
            .segments
            .iter()
            .last()
            .is_some_and(|s| s.ident.eq("Option")),
        _ => false,
    };
    let ident = p.ident;
    let ident_to_string = p.attrs.rename_str(ident);
    let value = if contain_self {
        quote!(self.#ident)
    } else {
        quote!(#ident)
    };
    if is_option {
        return quote! {
            if let Some(v) = #value {
                props.push(amplitude::event::Property {
                    name: #ident_to_string.to_owned(),
                    value: v.into(),
                });
            }
        };
    }
    quote! {
        props.push(amplitude::event::Property {
            name: #ident_to_string.to_owned(),
            value: #value.into(),
        });
    }
}

pub fn into_event_props<'a, I: Iterator<Item = &'a Ident>>(mut fields: I) -> TokenStream {
    // Must be at least one I think handle error?
    let first = fields.next().unwrap();
    let props = quote! {
        {
            let mut props = amplitude::Event::into_event_props(#first);
            #(props.extend(amplitude::Event::into_event_props(#fields));)*
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
