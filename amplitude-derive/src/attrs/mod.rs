use std::collections::HashMap;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, spanned::Spanned, AttrStyle, Attribute, Expr, Lit, Path, Token};

use self::symbol::{Symbol, AMPLI, RENAME};

pub mod symbol;

/// Attribute Options corresponding to amplitude derive options.
/// See [Symbol] for more information
pub struct AttrOptions(HashMap<Symbol, AttrOption>);

impl AttrOptions {
    pub fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut options = HashMap::new();
        for att in attrs.iter() {
            if let Some(opts) = AttrOption::parse(att) {
                options.extend(opts?);
            }
        }
        Ok(Self(options))
    }

    pub fn get(&self, s: &Symbol) -> Option<&AttrOption> {
        self.0.get(s)
    }
}

pub struct AttrOption {
    pub span: Span,
    pub str_value: Option<String>,
}

impl AttrOption {
    pub fn parse(att: &Attribute) -> Option<syn::Result<Vec<(Symbol, Self)>>> {
        if !matches!(att.style, AttrStyle::Outer) {
            return None;
        }
        if !att.path().is_ident(&AMPLI) {
            return None;
        }

        Some(Self::parse_ampli(att))
    }

    fn parse_ampli(att: &Attribute) -> syn::Result<Vec<(Symbol, Self)>> {
        let att_options: Punctuated<Expr, Token![,]> =
            att.parse_args_with(Punctuated::parse_terminated)?;
        let options: Vec<(Symbol, AttrOption)> = att_options
            .into_iter()
            .map(|e| match e {
                Expr::Assign(e) => {
                    let path = extract_path(&e.left)?;
                    let lit = extract_lit(&e.right)?;
                    Self::parse_assign(path, lit)
                }
                _ => Err(syn::Error::new(
                    e.span(),
                    "unrecognized amplitude attribute option",
                )),
            })
            .collect::<syn::Result<_>>()?;
        if options.is_empty() {
            return Err(syn::Error::new(
                att.span(),
                "must have at least one option for ampli attribute",
            ));
        }

        Ok(options)
    }

    fn parse_assign(path: &Path, lit: &Lit) -> syn::Result<(Symbol, Self)> {
        if path.is_ident(&RENAME) {
            let s = match lit {
                Lit::Str(s) => s.value(),
                _ => Err(syn::Error::new(lit.span(), "rename must be a string"))?,
            };
            return Ok((
                RENAME,
                Self {
                    span: path.span(),
                    str_value: Some(s),
                },
            ));
        }
        Err(syn::Error::new(
            path.span(),
            "unrecognized amplitude attribute option path",
        ))
    }
}

fn extract_path(e: &Expr) -> syn::Result<&Path> {
    match e {
        Expr::Path(p) => Ok(&p.path),
        _ => Err(syn::Error::new(e.span(), "must be a path")),
    }
}

fn extract_lit(e: &Expr) -> syn::Result<&Lit> {
    match e {
        Expr::Lit(l) => Ok(&l.lit),
        _ => Err(syn::Error::new(e.span(), "must be a literal")),
    }
}
