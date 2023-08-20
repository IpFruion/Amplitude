use syn::Ident;

/// A representation of the `Symbol`s contained in the `amplitude` project.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(&'static str);

pub const AMPLI: Symbol = Symbol("ampli");
pub const RENAME: Symbol = Symbol("rename");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}
