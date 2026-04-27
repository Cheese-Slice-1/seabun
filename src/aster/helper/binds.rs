use crate::aster::helper::{single::parse_val, errors::{malformed, unimp}};
use crate::def::{Token, TokenKind, Expr, BindKind};

use ecow::{EcoVec, EcoString};

// parses a binding
/// takes two parameters:
/// - "tokens": the vector of tokens that conform the bind
/// - "bind_kind": the chosen bind type as a bool pair representing (IS_MUT, IS_DEF)
fn parse_bind(tokens: EcoVec<Token>, bind_kind: BindKind) -> Expr {
    // line and column of malformed/unknown token
    let errpos = tokens[0].pos;

    // "parts" are the declaration's sides
    let parts = tokens
        .get(1..tokens.len())
        .unwrap_or_else(|| malformed("declaration", errpos))
        .split(|tok| tok.kind == TokenKind::EqSign)
        .collect::<EcoVec<_>>();
  
    /*
    for &part in &parts {
        println!("contains:\n{:#?}\n", part);
        println!("is type:\n{}", get_t!(part));
        println!();
    }
    */

    // return Expr::Empty;

    // if there's nothing between let/mut/def and "=", SCREAAAM "malformed declaration"
    if parts.get(0)
        .unwrap_or_else(|| malformed("declaration", errpos))
        .is_empty()
    {
        malformed("declaration", errpos);
    }
  
    let (name, var_kind, value) = (
        parts
            .get(0) // parts[0] contains both name and type
            .unwrap_or_else(|| malformed("declaration (missing name and type)", errpos))
            .get(0), // variable name; obligatory
        parts
            .get(0)
            .unwrap_or_else(|| malformed("declaration (missing name and type)", errpos))
            .get(1..), // variable type
        parts
            .get(1) // variable value
            .cloned()
    );
  
    let holds = match var_kind {
        Some(toks) => to_varkind(EcoVec::from(toks), errpos),
        _ => VarKind::Unknown,
    };

    //println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);
  
    Expr::Bind {
        id: name
            .unwrap()
            .literal
            .clone(),
        kind: (bind_kind, holds),
        val: match bind_kind {
            BindKind::Define => Box::new(Expr::Empty),
            _ => Box::new(parse_val(
                (*value.unwrap_or_else(|| malformed("declaration", errpos)))
                    .into(), // sorry for the nesting ;^;
                errpos,
                false
            )),
        }
    }
}

fn parse_rebind(_tokens: EcoVec<Token>) -> Expr {
    unimp()
}

