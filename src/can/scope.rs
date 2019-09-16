use can::symbol::Symbol;
use collections::ImMap;
use ident::Ident;
use region::Region;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    pub idents: ImMap<Ident, (Symbol, Region)>,
    symbol_prefix: String,
    next_unique_id: u64,
}

impl Scope {
    pub fn new(symbol_prefix: String, declared_idents: ImMap<Ident, (Symbol, Region)>) -> Scope {
        Scope {
            symbol_prefix,

            // This is used to generate unique names for anonymous closures.
            // It always begins at 0.
            next_unique_id: 0,

            idents: declared_idents,
        }
    }

    pub fn symbol(&self, name: &str) -> Symbol {
        Symbol::new(&self.symbol_prefix, name)
    }

    pub fn gen_unique_symbol(&mut self) -> Symbol {
        self.next_unique_id = self.next_unique_id + 1;

        Symbol::new(&self.symbol_prefix, &self.next_unique_id.to_string())
    }
}
