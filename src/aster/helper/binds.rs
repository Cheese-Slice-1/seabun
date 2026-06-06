use crate::aster::helper::{
    errors::{malformed, unimp},
    single::{parse_val, to_varkind},
};
use crate::def::{BindKind, Expr, NBit, Token, TokenKind, VarKind};

use ecow::{EcoString, EcoVec};

/// parses a binding.
/// takes two parameters:
/// - "tokens": the vector of tokens that conform the bind (not including the Dot token)
/// - "bind_kind": the chosen bind type
pub fn parse_bind(tokens: EcoVec<Token>, bind_kind: BindKind) -> (usize, Expr) { // usize = length of the code chunk
    use crate::get_t;
    // line and column of malformed/unknown token
    let errpos = tokens[0].pos;

    // "parts" are the declaration's sides
    let parts = tokens
        .get(1..tokens.len())
        .unwrap_or_else(|| malformed("declaration", errpos))
        .splitn(2, |tok| tok.kind == TokenKind::EqSign)
        .collect::<EcoVec<_>>();


    for &part in &parts {
        println!("contains:\n{:#?}\n", part);
        println!("is type:\n{}", get_t!(part));
        println!();
    }


    // return Expr::Empty;

    // if there's nothing between let/mut/def and "=", SCREAAAM "malformed declaration"
    if parts
        .first()
        .unwrap_or_else(|| malformed("declaration", errpos))
        .is_empty()
    {
        malformed("declaration", errpos);
    }

    let (name, var_kind, value) = (
        parts
            .first() // parts[0] contains both name and type
            .unwrap_or_else(|| malformed("declaration (missing name and type)", errpos))
            .first(), // variable name; obligatory
        parts
            .first()
            .unwrap_or_else(|| malformed("declaration (missing name and type)", errpos))
            .get(1..), // variable type
        parts
            .get(1) // variable value
            .cloned(),
    );

    let holds = match var_kind {
        Some(toks) => to_varkind(EcoVec::from(toks), errpos),
        _ => VarKind::Unknown,
    };

    //println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);

    let value = match bind_kind {
        BindKind::Define => (Box::new(Expr::Empty), 0),
        _ => Box::new(parse_val(
            (*value.unwrap_or_else(|| malformed("declaration", errpos))).into(), // sorry for the nesting ;^;
            errpos,
        )),
    };

    (
        0,
        Expr::Bind {
            id: name.unwrap().literal.clone(),
            kind: (bind_kind, holds),
            val: value,
            },
        }
    )
}

pub fn parse_rebind(_tokens: EcoVec<Token>) -> Expr {
    unimp(crate::def::CodePos(0, 0))
}

