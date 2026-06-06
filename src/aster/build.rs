// TODO: WHY DOESN'T IT FUCKING WORK I'M GONNA CRY WHATTTTTTTTTTTT (QnQc)
// someone help I BEG OF YOU :""/

//use std::collections::HashMap;
use ecow::{EcoString, EcoVec /*, eco_vec*/};

use crate::aster::helper::{
    binds::{parse_bind, parse_rebind},
    errors::*,
    single::to_varkind,
};
use crate::def::*;

// TODO: defined ids; move it to where it belongs!!
//static mut BINDINGS: EcoVec<(String, Expr)> = eco_vec![];

/// creates a very primitive ast
pub fn make_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
    let mut i: usize = 0; // boring index
    let mut res = EcoVec::<Expr>::new(); // the resulting AST (nodes)ss

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
                let bind: EcoVec<Token> = tokens[i..].into(); // let-related chunk
                    //.iter()
                    //.take_while(|tok| tok.kind != TokenKind::ExprEnd)
                    //.cloned()
                    //.collect::<EcoVec<Token>>();

                let bind_kind = match &literal[..] {
                    // NOTE: don't use kind. do not. please.
                    "mut" => BindKind::MutValue, // mutable
                    "let" => BindKind::Value,    // mutablen't (badum tsss)
                    "def" => BindKind::Define,   // mutablen't and a "typedef" (b a d u m   t s s s)
                    _ => panic!("this shouldn't happen!! wtf!!!!"),
                };

                let (length, r#final) = parse_bind(
                    bind, // tokens that conform th declaration
                    // always Let or Mut. nothing else should be possible
                    // if there's something else blame it on me or the lexer
                    // cuz lil bro shouldn't be doing that...
                    bind_kind,
                );
                i += length;
                
                println!("{:#?}", r#final);
                
                stop_here("fixing the implementation of the aster module");
                
                res.push(r#final);
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
            } => unknown(literal, *pos),
            
            /* NOTHING */
            _ => {}
        }
        
        //println!("{:#?}\n", res.last());
        
        i += 1;
    }

    res
}

/*
pub fn generate_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
    unimplemented!(); // TODO: clean up AST and resolve datatypes, expressions, etc. here
}
*/

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
