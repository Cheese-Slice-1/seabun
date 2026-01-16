use ecow::{EcoString, EcoVec};

extern crate logos;
extern crate chumsky;

use logos::{Logos, Lexer, Skip};

// use chumsky::prelude::*;

/// a seabun token; contains all necessary information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub literal: EcoString,
	pub span: (usize, usize),
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
	/* SYMBOL-BASED TOKENS */

	#[regex(r#"[#][^\x00-\x1F]*"#, priority=110)]
	Comment, // #a line comment

	#[regex(r#"([/][*]){1}([^\x00-\x09\x0C-\x1F]|\n)*([*][/]){1}"#, priority=100)]
	EnclosedComment, // /* an enclosed comment */
	
	#[regex(r#"#![^\x00-\x1F]+?"#, priority=120)]
	Doc,

	#[regex(r#"\."#, priority=100)]
	ExprEnd, // end of an expression

	#[regex(",", priority=100)]
	Comma,

	#[regex("=", priority=100)]
	EqSign,

	#[regex(":", priority=100)]
	Colon, // separates name from parameter/argument list
	
	#[regex("[!]", priority=100)]
	Bang, // ends parameter/argument list

	#[regex(";", priority=100)]
	Semicolon, // also ends parameter/argument list

	#[regex(r"[(]", priority=100)]
	LParen, // nested expr start
	
	#[regex(r"[)]", priority=100)]
	RParen, // nested expr end

	#[regex(r"\[", priority=100)]
	LBracket, // nested expr start
	
	#[regex(r"\]", priority=100)]
	Rbracket, // nested expr end

	#[regex(r"[{]", priority=100)]
	LBrace, // block start

	#[regex(r"[}]", priority=100)]
	RBrace, // block end

	#[regex(r"[{]{2}", priority=101)]
	LDblBrace, // tuple literal start

	#[regex(r"[}]{2}", priority=101)]
	RDblBrace, // tuple literal end

	#[regex(r"[+]", priority=100)]
	Plus,

	#[regex(r"-", priority=100)]
	Minus,

	#[regex(r"[*]", priority=100)]
	Star,

	#[regex(r"/", priority=100)]
	Slash,

	#[regex(r"\^", priority=100)]
	Caret,

	#[regex(r"%", priority=100)]
	Percent,

	/* KEYWORS-BASED TOKENS */

	// #[regex("->", priority=100)]
	#[regex("as", priority=60)]
	As, // casting
	
	#[regex("let", priority=60)]
	Let, // immutable variable declaration

	#[regex("var", priority=60)]
	Var, // mutable variable declaration

	#[regex("def", priority=60)]
	Def, // type definition
	
	#[regex("fun", priority=60)]
	Fun, // function literal; fun name: parameter type, ...!

	#[regex("rec", priority=60)]
	Rec, // record literal; rec: field type, ...!
	
	#[regex(r"if", priority=60)]
	If,
	
	#[regex(r"elif", priority=60)]
	Elif,
	
	#[regex(r"else", priority=60)]
	Else,
	
	#[regex(r"(true|false){1}", priority=70)]
	Bln,
	
	/* NON-KEYWORD-BASED TOKENS */
	
	#[regex(r"[^\d\x00-\x1F][a-zA-Z_][\da-zA-Z_]*", priority=40)]
	Word, // foo, bar_, _baz, bar2, seabun
	
	#[regex(r"\d+", priority=20)]
	Num, // 1, 2, 3, 4
	
	#[regex(r"[\d]*d[\d]+", priority=20)]
	Dot, // 1d5, d103, -9d9
	
	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))*""#, priority=20)]
	Str, // "hola", "HOLA", "HoLa123", "\""
	
	// ONE character or escape
	#[regex(r#"'([^'\\\x00-\x1F]|\\(['\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))?'"#, priority=20)]
	Chr, // 'c', '\u6F', '\u1234'
	
	/* ERROR TYPE REPRESENTING (line, column) */
	
	Error ((usize, usize)),
}

/// converts lex's captures to "Token"s
pub fn tokenize(lex: &mut Lexer<TokenKind>) -> EcoVec<Token> {
	lex.clone()
		.spanned() // gives (kind, span)
		.map(|el| { // el = one (kind, span) pair
			lex.next(); // advance lexer to get the slices
	
			//println!("token:\n{}\n----------", lex.slice()); // visualize current slice
	
			Token {
				kind: el.0.unwrap_or_else( // token kind start
					|_| {
						let line = lex.extras.0;
						let column = lex.span().start - lex.extras.1;
						TokenKind::Error((line, column))
					}
				), // token kind end
				
				literal: lex.slice().trim().into(), // literal
				
				span: (el.1.start, el.1.end) // span
			}
		})
		.filter(|el| { el.kind != TokenKind::Comment && el.kind != TokenKind::EnclosedComment })
		.collect()
}
