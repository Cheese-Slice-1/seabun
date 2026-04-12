// TODO: WHY DOESN'T IT FUCKING WORK I'M GONNA CRY WHATTTTTTTTTTTT (QnQc)
// someone help I BEG OF YOU :""/

//use std::collections::HashMap;
use ecow::{EcoString, EcoVec /*, eco_vec*/};

use crate::aster::helper::errors::*;
use crate::def::{BindKind, CodePos, Expr, Token, TokenKind, VarKind};

// TODO: defined ids; move it to where it belongs!!
//static mut BINDINGS: EcoVec<(String, Expr)> = eco_vec![];

/// creates a very primitive ast
pub fn primitive_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
    let mut i: usize = 0; // boring index
    let mut res: EcoVec<Expr> = EcoVec::new(); // the resulting AST (nodes)

    while i < tokens.len() {
        match &tokens[i] {
            /* DECLARATIONS */
            Token {
                kind: TokenKind::Let,
                literal,
                ..
            }
            | Token {
                kind: TokenKind::Mut,
                literal,
                ..
            }
            | Token {
                kind: TokenKind::Def,
                literal,
                ..
            } => {
                let bind: EcoVec<Token> = tokens[i..] // let-related chunk
                    .iter()
                    .take_while(|tok| tok.kind != TokenKind::ExprEnd)
                    .cloned()
                    .collect::<EcoVec<Token>>();
                i += bind.len() - 1;

                let bind_params = match &literal[..] {
                    // NOTE: don't use kind. do not. please.
                    "mut" => BindKind::MutValue, // mutable
                    "let" => BindKind::Value,    // mutablen't (badum tsss)
                    "def" => BindKind::Define,   // mutablen't and a "typedef" (b a d u m   t s s s)
                    _ => panic!("this shouldn't happen!! wtf!!!!"),
                };

                res.push(parse_bind(
                    bind, // tokens that conform th declaration
                    // always Let or Mut. nothing else should be possible
                    // if there's something else blame it on me or the lexer
                    // cuz lil bro shouldn't be doing that...
                    bind_params,
                ));
            }

            Token {
                kind: TokenKind::Word,
                literal,
                ..
            } if tokens[i + 1].kind == TokenKind::EqSign => {
                let rebind: EcoVec<Token> = tokens[i..]
                    .iter()
                    .take_while(|tok| tok.kind != TokenKind::ExprEnd)
                    .cloned()
                    .collect::<EcoVec<Token>>();
                i += rebind.len() - 1;

                res.push(parse_rebind(rebind));
            }

            /* ERROR */
            Token {
                kind: TokenKind::Error,
                literal,
                pos,
                ..
            } => unknown(literal, pos),

            /* NOTHING */
            _ => {}
        }

        //println!("{:#?}\n", res.last());

        i += 1;
    }

    res
}

/*
pub fn advanced_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
    unimplemented!(); // TODO: clean up AST and resolve datatypes, expressions, etc. here
}
*/

// AST-RELATED FUNCTIONS

/// parses a binding
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
    if parts
        .get(0)
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
            .cloned(),
    );

    let holds = match var_kind {
        Some(toks) => to_varkind(EcoVec::from(toks), errpos),
        _ => VarKind::Unknown,
    };

    //println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);

    Expr::Bind {
        id: name.unwrap().literal.clone(),
        kind: (bind_kind, holds),
        val: match bind_kind {
            BindKind::Define => Box::new(Expr::Empty),
            _ => Box::new(parse_val(
                (*value.unwrap_or_else(|| malformed("declaration", errpos))).into(), // sorry for the nesting ;^;
                errpos,
                false,
            )),
        },
    }
}

fn parse_rebind(_tokens: EcoVec<Token>) -> Expr {
    unimp()
}

/* SIMPLE EXPRESSIONS GENERATORS */

/* UTILS */

pub fn to_varkind(toks: EcoVec<Token>, pos: CodePos) -> VarKind {
    // predefined literals
    if toks.len() < 2 {
        let Some(tok) = toks.get(0) else {
            return VarKind::Unknown;
        };
        return match &tok.literal[..] {
            "num" => VarKind::Num,
            "dot" => VarKind::Dot,
            "chr" => VarKind::Chr,
            "str" => VarKind::Str,
            "bln" => VarKind::Bln,
            _ => VarKind::Unknown, // non-primitive (like custom types, tuples, arrays and records)
        };
    }

    // TODO: implement complex types like funs, recs, arrays, etc.
    // very important so remember eh?

    let mut ret = VarKind::Unknown;

    for (i, tok) in toks.iter().enumerate() {
        println!("{i}) {tok:#?}");
    }

    unimp()
}

/*
    implementation should turn (simplified):
    [
        Token {kind: TokenKind::Let, ..},
        Token {kind: TokenKind::Name, literal: "x".into(), ..},
        Token {kind: TokenKind::EqSign, ..}
        Token {kind: TokenKind::Num, literal: "5".into(), ..},
        Token {kind: TokenKind::Plus, ..}
        Token {kind: TokenKind::Num, literal: "5".into(), ..},
        Token {kind: TokenKind::ExprEnd, ..}
    ]
    into:
    [
        Expr::Bind {
            id: "x".into(),
            kind: VarKind::Unknown,
            val: Box::new(Expr::Op {
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(5)),
                op: '+',
            }),
            is_mut: false,
        },
    ]
    which would then resolve as:
    [
        Expr::Bind {
            id: "x".into(),
            kind: VarKind::Num,
            val: Box::new(Expr::Num(10)),
            is_mut: false,
        },
    ]
*/
