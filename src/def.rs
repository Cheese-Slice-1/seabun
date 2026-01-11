use std::collections::HashMap;

extern crate logos;
extern crate chumsky;

use logos::{Logos, Lexer, Skip};

// use chumsky::prelude::*;

/// a seabun token; contains all necessary information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub literal: String,
	pub span: std::ops::Range<usize>,
}

/*
	PRIORIDADES:
	0. comentarios
	1. simbolos
	2. palabras clave
	3. IDs y tipos
	4. literales
*/
#[derive(Logos)]
#[derive(Clone, Debug, PartialEq)]
#[logos(extras=(usize, usize))]
#[logos(skip (r#"\s+?"#, |l| {
	if l.slice().chars().any(|c| c == '\n') {
		l.extras.0 += 1;
		l.extras.1 = l.span().end;
	}
	Skip
}))]
pub enum TokenKind {
	#[regex(r#"[#][^\x00-\x1F]+?"#, priority=110)]
	Comment,
	
	#[regex(r#"#![^\x00-\x1F]+?"#, priority=120)]
	Doc,

	#[regex(r#"\."#, priority=100)]
	ExprEnd, // end of an expression

	#[regex(",", priority=100)]
	Comma,

	// #[regex("->", priority=100)]
	#[regex("as", priority=60)]
	As, // casting
	
	#[regex("let", priority=60)]
	Let, // immutable variable declaration

	#[regex("var", priority=60)]
	Var, // mutable variable declaration

	#[regex("=", priority=100)]
	EqSign,
	
	#[regex("fun", priority=60)]
	Fun, // function declaration
	
	#[regex(":", priority=100)]
	LArgs, // separates name from args
	
	#[regex("[!]", priority=100)]
	#[regex(";", priority=100)]
	RArgs, // ends args section

	#[regex("[{]", priority=100)]
	LBrace, // block start

	#[regex("[}]", priority=100)]
	RBrace, // also ends a statement (block body)

	#[regex("[{]{2}", priority=101)]
	LDblBrace, // tuple start

	#[regex("[}]{2}", priority=101)]
	RDblBrace, // tuple end
	
	#[regex("if", priority=60)]
	If,
	
	#[regex("elif", priority=60)]
	Elif,
	
	#[regex("else", priority=60)]
	Else,
	
	#[regex("[(]", priority=100)]
	LParen, // nested expr start
	
	#[regex("[)]", priority=100)]
	RParen, // nested expr end

	#[regex("[+]", priority=100)]
	Plus,

	#[regex("-", priority=100)]
	Minus,
	
	#[regex(r#"[^\d\x00-\x1F][a-zA-Z_][\da-zA-Z_]*"#, priority=40)]
	Name, // foo, bar_, _baz, bar2, seabun
	
	#[regex(r#"[-]?\d+"#, priority=20)]
	Num, // 1, 2, 3, 4
	
	#[regex(r#"[-]?[\d]*d[\d]+"#, priority=20)]
	Dot, // 1d5, d103, -9d9
	
	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))*""#, priority=20)]
	Str, // "hola", "HOLA", "HoLa123", "\""

	// ONE character or escape
	#[regex(r#"'([^'\\\x00-\x1F]|\\(['\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))?'"#, priority=20)]
	Chr, // 'c', '\u6F', '\u1234'
	
	#[regex(r#"true|false"#, priority=60)]
	Bln,
	
	Error ((usize, usize)),
}

pub fn tokenize(lex: &mut Lexer<TokenKind>) -> Vec<Token> {
	lex.clone()
		.spanned() // gives (kind, span)
		.map(|el| { // el = one (kind, span) pair
			lex.next(); // advance lexer to get the slices
			Token {
				kind: el.0.unwrap_or_else( // token kind start
					|_| {
						let line = lex.extras.0;
						let column = lex.span().start - lex.extras.1;
						TokenKind::Error((line, column))
					}
				), // token kind end

				literal: lex.slice().trim().to_owned(), // literal

				span: el.1 // span
			}
		})
		.filter(|el| el.kind != TokenKind::Comment)
		.collect()
}
