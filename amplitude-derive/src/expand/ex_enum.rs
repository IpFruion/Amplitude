use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, Fields, Generics, Ident, Variant};

use crate::attrs::AttrOptions;

use super::prop::{event_props, into_event_props, FieldProp};

pub fn event_enum(ident: Ident, generics: Generics, s: DataEnum) -> syn::Result<TokenStream> {
    let mut name_match_variants = Vec::new();
    let mut into_event_props_variants = Vec::new();
    for v in s.variants.iter() {
        let (name_variant, props_variant) = variant_match_builder(v)?;
        name_match_variants.push(name_variant);
        into_event_props_variants.push(props_variant);
    }

    Ok(quote! {
        impl amplitude::Event for #ident #generics {
            fn name(&self) -> &str {
                match self {
                    #(#name_match_variants,)*
                }
            }
            fn into_event_props(self) -> Vec<amplitude::event::Property> {
                match self {
                    #(#into_event_props_variants,)*
                }
            }
        }
    })
}

struct VariantMatch {
    left_hand_side: TokenStream,
    right_hand_side: TokenStream,
}

impl ToTokens for VariantMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let left_hand_side = &self.left_hand_side;
        let right_hand_side = &self.right_hand_side;
        let token = quote! {
            #left_hand_side => #right_hand_side
        };
        token.to_tokens(tokens);
    }
}

impl VariantMatch {
    fn name_variant(
        left_hand_match: TokenStream,
        ident: &Ident,
        attrs: &AttrOptions,
    ) -> VariantMatch {
        let ident_to_string = attrs.rename_str(ident);
        VariantMatch {
            left_hand_side: left_hand_match,
            right_hand_side: quote! { #ident_to_string },
        }
    }

    fn empty_fields(ident: &Ident, attrs: &AttrOptions) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };

        let name_variant = Self::name_variant(left_hand_match.clone(), ident, attrs);
        let props_variant = VariantMatch {
            left_hand_side: left_hand_match,
            right_hand_side: quote!(Vec::new()),
        };
        (name_variant, props_variant)
    }

    fn named_fields<'a, F: Iterator<Item = FieldProp<'a>>>(
        ident: &Ident,
        f: F,
        attrs: &AttrOptions,
    ) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };
        let mut ignored = Vec::new();
        let mut idents = Vec::new();
        let mut field_props = Vec::new();
        for p in f {
            let ident = p.ident;
            ignored.push(quote!(#ident: _));
            idents.push(ident);
            field_props.push(p);
        }

        let name_variant = Self::name_variant(
            quote! {
                #left_hand_match{
                    #(#ignored,)*
                }
            },
            ident,
            attrs,
        );
        let props = event_props(field_props.iter(), false);
        let props_variant = Self {
            left_hand_side: quote! {
               #left_hand_match{
                    #(#idents,)*
                }
            },
            right_hand_side: props,
        };
        (name_variant, props_variant)
    }

    fn unamed_fields(
        ident: &Ident,
        amount: usize,
        attrs: &AttrOptions,
    ) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };
        let (ignored, named): (Vec<_>, Vec<_>) = (0..amount)
            .map(|i| (quote!(_), gen_unamed_field_ident(i)))
            .unzip();
        let name_variant = Self::name_variant(
            quote! {
                #left_hand_match(#(#ignored,)*)
            },
            ident,
            attrs,
        );

        let props = into_event_props(named.iter());
        let props_variant = Self {
            left_hand_side: quote! {
                #left_hand_match(#(#named,)*)
            },
            right_hand_side: props,
        };
        (name_variant, props_variant)
    }
}

fn variant_match_builder(v: &Variant) -> syn::Result<(VariantMatch, VariantMatch)> {
    let attrs = AttrOptions::parse(&v.attrs)?;
    let ident = &v.ident;
    Ok(match &v.fields {
        Fields::Unit => VariantMatch::empty_fields(ident, &attrs),
        Fields::Named(f) => {
            let fields = f
                .named
                .iter()
                .map(FieldProp::try_from)
                .collect::<syn::Result<Vec<_>>>()?;
            VariantMatch::named_fields(ident, fields.into_iter(), &attrs)
        }
        Fields::Unnamed(f) => VariantMatch::unamed_fields(ident, f.unnamed.len(), &attrs),
    })
}

fn gen_unamed_field_ident(i: usize) -> Ident {
    let mut i = i;
    let mut s = String::new();
    loop {
        let c = (i % 26) as u8 + b'a';
        s.push(c as char);
        if i / 26 == 0 {
            break;
        }
        i /= 26;
        i -= 1;
    }
    let s: String = s.chars().rev().collect();
    format_ident!("{}", s)
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use super::gen_unamed_field_ident;

    #[test]
    pub fn ident_unamed_field() {
        let i = gen_unamed_field_ident(0);
        assert_eq!(i, format_ident!("a"));
        let i = gen_unamed_field_ident(1);
        assert_eq!(i, format_ident!("b"));
        let i = gen_unamed_field_ident(25);
        assert_eq!(i, format_ident!("z"));
        let i = gen_unamed_field_ident(26);
        assert_eq!(i, format_ident!("aa"));
        let i = gen_unamed_field_ident(27);
        assert_eq!(i, format_ident!("ab"));
        let i = gen_unamed_field_ident(28);
        assert_eq!(i, format_ident!("ac"));
        let i = gen_unamed_field_ident(51);
        assert_eq!(i, format_ident!("az"));
        let i = gen_unamed_field_ident(52);
        assert_eq!(i, format_ident!("ba"));
    }
}
