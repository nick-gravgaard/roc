extern crate bumpalo;
extern crate roc;

use self::bumpalo::Bump;
use roc::can;
use roc::can::expr::Expr;
use roc::can::problem::Problem;
use roc::can::procedure::Procedure;
use roc::can::symbol::Symbol;
use roc::can::Output;
use roc::collections::{ImMap, MutMap};
use roc::ident::Ident;
use roc::parse;
use roc::parse::ast::{self, Attempting};
use roc::parse::parser::{Fail, Parser, State};
use roc::region::{Located, Region};

pub fn parse_with<'a>(arena: &'a Bump, input: &'a str) -> Result<ast::Expr<'a>, Fail> {
    let state = State::new(&input, Attempting::Module);
    let parser = parse::expr();
    let answer = parser.parse(&arena, state);

    answer.map(|(expr, _)| expr).map_err(|(fail, _)| fail)
}

#[allow(dead_code)]
pub fn can_expr(expr_str: &str) -> (Expr, Output, Vec<Problem>, MutMap<Symbol, Procedure>) {
    can_expr_with(
        &Bump::new(),
        "blah",
        expr_str,
        &ImMap::default(),
        &ImMap::default(),
    )
}

#[allow(dead_code)]
pub fn can_expr_with(
    arena: &Bump,
    name: &str,
    expr_str: &str,
    declared_idents: &ImMap<Ident, (Symbol, Region)>,
    declared_variants: &ImMap<Symbol, Located<Box<ast::VariantName>>>,
) -> (Expr, Output, Vec<Problem>, MutMap<Symbol, Procedure>) {
    let expr = parse_with(&arena, expr_str).unwrap_or_else(|_| {
        panic!(
            "can_expr_with() got a parse error when attempting to canonicalize:\n\n{:?}",
            expr_str
        )
    });

    let home = "Test".to_string();
    let (loc_expr, output, problems, procedures) = can::canonicalize_declaration(
        home,
        name,
        Located::new(0, 0, 0, 0, expr),
        declared_idents,
        declared_variants,
    );

    (loc_expr.value, output, problems, procedures)
}

// pub fn mut_map_from_pairs<K, V, I>(pairs: I) -> MutMap<K, V>
// where
//     I: IntoIterator<Item = (K, V)>,
//     K: Hash + Eq,
// {
//     let mut answer = MutMap::default();

//     for (key, value) in pairs {
//         answer.insert(key, value);
//     }

//     answer
// }
