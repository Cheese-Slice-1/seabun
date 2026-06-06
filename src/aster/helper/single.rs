use crate::aster::helper::errors::*;
use crate::def::{CodePos, Expr, NBit, Token, TokenKind, VarKind};

use ecow::{EcoString, EcoVec};
use unescaper::unescape;

// URGENT: migrate to non-recursive asap
/// precedence-based non-composite expression parser
pub fn parse_val(value: EcoVec<Token>, errpos: CodePos) -> Box<Expr> {
    // expression to return
    let mut res = Expr::Empty;

    if value.is_empty() {
        return Box::new(res);
    }

    // single token expression
    // URGENT: remove in favor of purely loop-based parsing
    if value.len() == 1 {
        return match &value[0] {
            // kind (Expr) isn't Cow (clone-on-write)

            // a variable/type/etc. name
            Token {
                kind: TokenKind::Word,
                literal,
                ..
            } => Expr::Name(literal.clone()),

            // a integer
            Token {
                kind: TokenKind::Num,
                literal,
                pos,
            } => Expr::Num(
                literal
                    .parse::<isize>()
                    .unwrap_or_else(|_| malformed("num literal", *pos)),
            ),

            // a float
            Token {
                kind: TokenKind::Dot,
                literal,
                pos,
            } => Expr::Dot(
                literal
                    .replace("d", ".")
                    .parse::<f64>()
                    .unwrap_or_else(|_| malformed("dot literal", *pos)),
            ),

            // a boolean
            Token {
                kind: TokenKind::Bln,
                literal,
                pos,
            } => Expr::Bln(match &literal[..] {
                "yes" | "true" => true,
                "no" | "false" => false,
                _ => malformed("bln literal", *pos),
            }),

            // a character literal
            Token {
                kind: TokenKind::Chr,
                literal,
                pos,
            } => {
                let raw: char = unescape(literal.get(1..literal.len() - 1).unwrap_or(r"\u0000"))
                    .unwrap_or_else(|_| malformed("chr literal", *pos))
                    .chars()
                    .next()
                    .unwrap_or('\x00');

                Expr::Chr(raw)
            }

            // do NOT try collapse them into one
            // it'll bring destruction to the compiler, and headaches to me
            Token {
                kind: TokenKind::Str,
                literal,
                pos,
            } => {
                let unescaped = unescape(
                    &unbun_str(TokenKind::Str, literal, *pos)[..]) // dumbass everything
                        .unwrap_or_else(|_| {
                            dumbass_compiler(TokenKind::Str, literal)
                        }
				);

                println!("{unescaped}");

                Expr::Str(unescaped.into())
            }
            Token {
                kind: TokenKind::RawStr,
                literal,
                pos,
            } => {
                let unescaped = unescape(
					&unbun_str(TokenKind::RawStr, literal, *pos)[..]) // dumbass everything
						.unwrap_or_else(|_| {
							dumbass_compiler(TokenKind::RawStr, literal)
						}
				);

                println!("{unescaped}");

                Expr::Str(unescaped.into())
            }

            Token {
                kind: TokenKind::RParen,
                ..
            } => malformed("expression (no opening parentheses)", errpos),

            _ => malformed("or unimplemented expression", errpos), // TODO: parse other single-token expressions
        };
    }

    let mut i: usize = 0;
    let mut expr_depth: usize = 0;
    let mut expr_stack = EcoVec::<Expr>::new();

    // NOTE: the idea is to make "res" recover with merges (like, an operation + value = operation with one operand)
    // instead of the previous recursive calls. that way it also makes it easier to parse sub-values withput worrying
    // about matching closing delimiters :3 (i think??)
    while i < value.len() {
        match &value[i] {
            // a parenthesized expression
            Token {
                kind: TokenKind::LParen,
                pos,
                ..
            } => {
                //let parenspan = value[i..] // sub tokens to parse
                //    .iter()
                //    .take_while(|tok| tok.kind != TokenKind::ExprEnd)
                //    .cloned()
                //    .collect::<EcoVec<Token>>();

                //i += parenspan.len();

                expr_stack.push(res);
                expr_depth += 1;
                res = Expr::Empty;
                // parse_val on everything until a ")"
            },

            Token {
                kind: TokenKind::RParen,
                pos,
                ..
            } => {
                if expr_depth < 1 {
                    malformed("parenthesized expression", *pos);
                }

                let previous = expr_stack.pop()
                    .unwrap_or_else(|| {
                        eprintln!("None on expr_stack after checking, wtf?");
                        std::process::exit(3)
                    });


            },

            Token {
                kind: TokenKind::Word,
                literal,
                ..
            } => res = Expr::Name(literal.clone()),

            Token {
                kind: TokenKind::Num,
                literal,
                pos,
            } => {
                expr_stack.push(Expr::Num(
                    literal
                        .parse::<i128>()
                        .unwrap_or_else(|_| malformed("num literal", *pos)),
                ));
            },

            Token {
                kind: TokenKind::Error,
                literal,
                pos,
            } => unknown(&value[i].literal, value[i].pos),

            Token {
                kind: TokenKind::ExprEnd,
                ..
            } => {
                if expr_depth < 1 {
                    return res;
                } else {
                    unimp(errpos);
                }
            },

            _ => malformed("expression (if malformed, probably missing a delimeter)", value[0].pos),
        }

        i += 1;
    }

    /*
        ideas:
            - parse tokens as if they were characters
            - no operator precedence; pure LtR with precedence for parentheses
            - LParen => recursive parse_val
            - RParen => forcibly return from parse_val
    */

    //unimplemented!();

    Box::new({
        if res.check() {
            return res;
        }else {
            return expr_stack
                .pop()
                .unwrap_or_else(|| stop_here("idk what's happening but the parser is doing Things TM"));
        }
    })
}

pub fn to_varkind(toks: EcoVec<Token>, errpos: CodePos) -> VarKind {
    // predefined literals
    if toks.len() < 2 {
        let Some(tok) = toks.first() else {
            return VarKind::Unknown;
        };

        return match &tok.literal[..] {
            "num" => VarKind::Num(NBit::TSize),
            "dot" => VarKind::Dot(NBit::TSize),
            "chr" => VarKind::Chr(NBit::T8),
            "str" => VarKind::Str(NBit::T8),
            "bln" => VarKind::Bln(NBit::T8),
            _ => VarKind::Unknown, // non-primitive (custom/alias)
        };
    }

    // TODO: implement complex types like funs, recs, arrays, etc.
    // very important so remember eh?

    let mut ret = VarKind::Unknown;

    for (i, tok) in toks.iter().enumerate() {
        println!("{i}) {tok:#?}");

        match tok {
            Token {
                kind: TokenKind::LBracket,
                pos,
                ..
            } => unimp(*pos),

            Token {
                kind: TokenKind::RBracket,
                pos,
                ..
            } => unimp(*pos),

            _ => unimp(errpos),
        }
    }

    unimp(pos)
}

pub fn unbun_str(kind: TokenKind, string: &EcoString, pos: CodePos) -> EcoString {
    match kind {
        TokenKind::Str => {
            let end = string.len() - 1;
            string
                .get(1..end)
                .unwrap_or_else(|| malformed("string literal", pos))
                .into()
        }
        TokenKind::RawStr => {
            let end = string.len() - 2;
            string
                .get(2..end)
                .unwrap_or_else(|| malformed("raw string literal", pos))
                .into()
        }
        _ => dumbass_compiler(kind, string),
    }
}

