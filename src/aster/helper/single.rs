use crate::aster::helper::errors::*;
use crate::def::{Token, TokenKind, CodePos, Expr};

use ecow::{EcoVec, EcoString};
use unescaper::unescape;

/// precedence-based non-composite expression parser
pub fn parse_val(value: EcoVec<Token>, errpos: CodePos, isnested: bool) -> Expr {
    // expression to return
    let mut res = Expr::Empty;

    if value.is_empty() {
        return res;
    }

    // single token expression
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
                    .nth(0)
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
                let unescaped = unescape(&unbun_str(TokenKind::Str, literal, *pos)[..]) // dumbass everything
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
            } => {
                if isnested {
                    res
                } else {
                    malformed("expression", errpos)
                }
            }

            _ => malformed("or unimplemented expression", errpos), // TODO: parse other single-token expressions
        };
    }

    let mut i: usize = 0;

    while i < value.len() {
        match value[i] {
            // a parenthesized expression
            Token {
                kind: TokenKind::LParen,
                pos,
                ..
            } => {
                let parenspan = value[i..] // sub tokens to parse
                    .iter()
                    .take_while(|tok| tok.kind != TokenKind::ExprEnd)
                    .cloned()
                    .collect::<EcoVec<Token>>();

                i += parenspan.len();

                let parenval = parse_val(parenspan, pos, true);

                res = parenval;
                // parse_val on everything until a ")"
            }

            Token {
                kind: TokenKind::RParen,
                pos,
                ..
            } => {
                if isnested {
                    return res;
                } else {
                    malformed("parenthesized expression", pos);
                }
            }

            Token {
                kind: TokenKind::Word,
                literal,
                ..
            } => res = Expr::Name(literal.clone()),

            _ => unknown(&value[i].literal, value[i].pos),
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

    res
}

pub fn unbun_str<'a>(kind: TokenKind, string: &EcoString, pos: CodePos) -> EcoString {
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
