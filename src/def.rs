use std::collections::HashMap;

extern crate logos;
extern crate chumsky;

use logos::{Logos, Lexer, Skip};

// use chumsky::prelude::*;

pub fn toprimitive(l: &mut Lexer<TokenKind>) -> VarKind {
	let lit = l
		.slice()
		.trim();
		
	// predefined literals
	match lit {
		"num" => VarKind::Num,
		"dot" => VarKind::Dot,
		"chr" => VarKind::Chr,
		"str" => VarKind::Str,
		"bln" => VarKind::Bln,
		_ => VarKind::Unknown, // non-primitive (like tuples, arrays and records)
	}
}

pub struct Token((TokenKind, String));


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
	
	#[regex(r#"#![^\x00-\x1F]+?"#, |doc| {
		doc
			.slice()
			.trim()
			.to_owned()
	}, priority=120)]
	Doc(String),

	#[regex(r#"\."#, priority=100)]
	ExprEnd, // end of an expression

	#[regex(",", priority=100)]
	Comma,

	// #[regex("->", priority=100)]
	#[regex("as", priority=100)]
	As, // casting
	
	#[regex("let", priority=60)]
	Let, // variable declaration

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
	
	#[regex(r#"[^\d\x00-\x1F][a-zA-Z_][\da-zA-Z_]*"#, |name| {
		name
			.slice()
			.trim()
			.to_owned()
	}, priority=40)]
	Name(String), // foo, bar_, _baz, bar2, seabun
	
	#[regex(r#"[-]?\d+"#, |catch| {
		catch.slice()
			.trim()
			.parse::<i64>()
			.unwrap()
	}, priority=20)]
	Num(i64), // 1, 2, 3, 4
	
	#[regex(r#"[-]?[\d]*d[\d]+"#, |catch| {
		catch.slice()
			.trim()
			.replace("d", ".")
			.parse::<f64>()
			.unwrap()
	}, priority=20)]
	Dot(f64), // 1d5, d103, -9d9
	
	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))*""#,
		|s| s.slice().trim()[1..s.slice().len()-1].to_owned(),
		priority=20)]
	Str(String), // "hola", "HOLA", "HoLa123", "\""

	// ONE character or escape
	#[regex(r#"'([^'\\\x00-\x1F]|\\(['\\bnfrt]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{2}))?'"#,
		|c| c.slice().trim()[1..c.slice().len()-1].to_owned(),
		priority=20)]
	Chr(String), // 'c', '\u6F', '\u1234'
	
	#[regex(r#"true|false"#, priority=60)]
	Bln(bool),
	
	Error((usize, usize)),
}

/// every possible variable kind in seabun
#[derive(Clone, Debug, PartialEq)]
pub enum VarKind {
	Num,
	Dot,
	Str,
	Chr,
	Bln,
	Arr(Box<VarKind>, usize),
	Tup(Vec<VarKind>, usize),
	Rec(Vec<VarKind>, usize),
	Unknown, // resolves when making AST; if not throws error
}

pub fn tokenize() -> Vec<Token> {
	
}
