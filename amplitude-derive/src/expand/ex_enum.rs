use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, Fields, Generics, Ident, Variant};

use super::prop::{event_props, into_event_props};

pub fn event_enum(ident: Ident, generics: Generics, s: DataEnum) -> syn::Result<TokenStream> {
    let (name_match_variants, into_event_props_variants): (Vec<_>, Vec<_>) = s
        .variants
        .iter()
        .map(|v| {
            let (name_variant, props_variant) = variant_match_builder(v);
            (name_variant, props_variant)
        })
        .unzip();

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
    fn name_variant(left_hand_match: TokenStream, ident: &Ident) -> VariantMatch {
        let ident_to_string = ident.to_string();
        let ident_to_string = quote! { #ident_to_string };
        VariantMatch {
            left_hand_side: left_hand_match,
            right_hand_side: ident_to_string,
        }
    }

    fn empty_fields(ident: &Ident) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };

        let name_variant = Self::name_variant(left_hand_match.clone(), ident);
        let props_variant = VariantMatch {
            left_hand_side: left_hand_match,
            right_hand_side: quote!(Vec::new()),
        };
        (name_variant, props_variant)
    }

    fn named_fields<'a, F: Iterator<Item = &'a Ident>>(
        ident: &Ident,
        f: F,
    ) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };
        let (ignored, named): (Vec<_>, Vec<_>) = f.map(|ident| (quote!(#ident: _), ident)).unzip();
        let name_variant = Self::name_variant(
            quote! {
                #left_hand_match{
                    #(#ignored,)*
                }
            },
            ident,
        );
        let props = event_props(named.iter().map(|i| *i), false);
        let props_variant = Self {
            left_hand_side: quote! {
               #left_hand_match{
                    #(#named,)*
                }
            },
            right_hand_side: props,
        };
        (name_variant, props_variant)
    }

    fn unamed_fields(ident: &Ident, amount: usize) -> (VariantMatch, VariantMatch) {
        let left_hand_match = quote! { Self::#ident };
        let (ignored, named): (Vec<_>, Vec<_>) = (0..amount)
            .map(|i| (quote!(_), gen_unamed_field_ident(i)))
            .unzip();
        let name_variant = Self::name_variant(
            quote! {
                #left_hand_match(#(#ignored,)*)
            },
            ident,
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

fn variant_match_builder(v: &Variant) -> (VariantMatch, VariantMatch) {
    let ident = &v.ident;
    match &v.fields {
        Fields::Unit => VariantMatch::empty_fields(ident),
        Fields::Named(f) => {
            VariantMatch::named_fields(ident, f.named.iter().map(|f| f.ident.as_ref().unwrap()))
        }
        Fields::Unnamed(f) => VariantMatch::unamed_fields(ident, f.unnamed.len()),
    }
}

fn gen_unamed_field_ident(i: usize) -> Ident {
    let mut i = i;
    let mut s = String::new();
    loop {
        let c = (i % 26) as u8 + 'a' as u8;
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
