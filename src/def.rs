use std::collections::HashMap;
use ecow::{EcoString, EcoVec};

extern crate logos;
extern crate chumsky;

use logos::{Logos, Lexer, Skip};

// use chumsky::prelude::*;

/// a seabun token; contains all necessary information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind, // what it is
	pub literal: EcoString, // raw form
	pub pos: (usize, usize), // position
}

/*
	PRIORIDADES:
	0. comentarios
	1. simbolos
	2. literales
	3. palabras clave
	4. IDs y tipos
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

	#[regex(r#";[^\x00-\x1F]*"#, priority=110)]
	Comment, // #a line comment
	
	#[regex(r";![^\x00-\x1F]+?", priority=120)]
	Doc,

	#[regex("[.]{1}", priority=100)]
	ExprEnd, // end of an expression

	#[regex(",", priority=100)]
	Comma,

	#[regex("=", priority=100)]
	EqSign,

	#[regex(":", priority=100)]
	Colon, // separates name from parameter/argument list
	
	#[regex("[!]", priority=100)]
	Bang, // ends parameter/argument list

	#[regex("[(]", priority=100)]
	LParen, // nested expr start
	
	#[regex("[)]", priority=100)]
	RParen, // nested expr end

	#[regex(r"\[", priority=100)]
	LBracket, // nested expr start
	
	#[regex(r"\]", priority=100)]
	Rbracket, // nested expr end

	#[regex("[{]", priority=100)]
	LBrace, // block start

	#[regex("[}]", priority=100)]
	RBrace, // block end

	#[regex("[{]{2}", priority=101)]
	LDblBrace, // tuple literal start

	#[regex("[}]{2}", priority=101)]
	RDblBrace, // tuple literal end

	#[regex("[+]", priority=100)]
	Plus,

	#[regex("-", priority=100)]
	Minus,

	#[regex("[*]", priority=100)]
	Star,

	#[regex("/", priority=100)]
	Slash,

	#[regex(r"\^", priority=100)]
	Caret,

	#[regex("%", priority=100)]
	Percent,

	#[regex("#", priority=100)]
	Hash,
	
	#[regex("@", priority=100)]
	AtSign,

	#[regex("->", priority=100)]
	Arrow, // will denote return type for funs and mabe smth more??

	/* KEYWORS-BASED TOKENS */

	// #[regex("->", priority=100)]
	#[regex("as", priority=60)]
	As, // casting
	
	#[regex("let", priority=60)]
	Let, // immutable binding declaration

	#[regex("mut", priority=60)]
	Mut, // mutable binding declaration

	#[regex("def", priority=60)]
	Def, // type definition
	
	#[regex("fun", priority=60)]
	Fun, // function literal; fun: parameter type, ... -> type

	#[regex("rec", priority=60)]
	Rec, // record literal; rec: field type, ...!
	
	#[regex("if", priority=60)]
	If,
	
	#[regex("elif", priority=60)]
	Elif,
	
	#[regex("else", priority=60)]
	Else,
	
	/* NON-KEYWORD-BASED TOKENS */
	
	#[regex(r"\d+", priority=40)]
	Num, // 1, 2, 3, 4
	
	#[regex(r"[\d]*d[\d]+", priority=40)]
	Dot, // 1d5, d103, -9d9
	
	#[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt]|u[a-fA-F0-9]{4}|x[a-fA-F0-9]{2}))+?""#, priority=40)]
	Str, // "hola", "HOLA", "HoLa123", "\""
	
	// ONE character or escape
	#[regex(r#"'([^'\\\x00-\x1F]|\\(['\\bnfrt]|u[a-fA-F0-9]{4}|x[a-fA-F0-9]{2}))'"#, priority=40)]
	Chr, // 'c', '\u6F', '\\', '\'', '\n'
	
	#[regex("true|false|yes|no", priority=40)]
	Bln, // true, false

	#[regex(r#"[^\d\s\x00-\x20.'"][^\s\x00-\x20.'"]*"#, priority=20)]
	Word, // foo, bar_, _baz, bar2, seabun
	
	/* ERROR TYPE REPRESENTING (line, column) */
	
	Error,
}

/// every possible Seabun expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	/* SIMPLE EXPRESSIONS */
	// these consist of a single non-self-referential value

	/// empty expression; throws an error in declarations
	Empty,

	/// integer
	Num (i64),
	
	/// float
	Dot (f64),
	
	/// string literal
	Str (EcoString),
	
	/// character literal
	Chr (char),

	/// boolean
	Bln (bool),

	/// variable/function/type name
	Name (EcoString),

	/* SIMPLE COMPUND EXPRESSIONS */
	/* they produce concrete value and kind*/

	/// a single block
	Block (EcoVec<Expr>),

	/// tuple
	Tup (EcoVec<Expr>),

	/// array
	Arr (EcoVec<Expr>, VarKind), // varkind = first value's varkind
	
	/// record
	Rec (HashMap<EcoString, Expr>),

	/// function literal; like fun: arg num! {}
	Fun { 
		kind: VarKind, // return type
		args: (usize, HashMap<EcoString, VarKind>), // argument types + names
		body: Box<Expr>, 
	},
	
	/// arithmetic operation
	Op { //x+y, x-y, x*y, x/y, x^x, x%x
		left: Box<Expr>,
		right:Box<Expr>,
		op: char,
	},

	/* COMPLEX COMPOUND EXPRESSIONS */
	/* they produce an Expr::Empty value and a VarType::Unknown kind (tl;dr: can't be values) */
	
	/// represents a new push to the stack or allocation on the heap
	Bind { // e.g. let s = "". -> Decl {id: "s".into(), kind: VarKind::Str, val: Box::new(Expr::Str("a".to_owned())), ismut: false}
		id: EcoString,
		kind: VarKind,
		val: Box<Expr>,
		ismut: bool,
	},

	/// represents a change to an already existing variable
	ReBind { // e.g. 
		id: EcoString,
		kind: VarKind,
		val: Box<Expr>,
	}
}

/// every possible variable kind in Seabun;
/// these only contain essential info
#[derive(Clone, Debug, PartialEq)]
pub enum VarKind {
	Num,
	Dot,
	Str,
	Chr,
	Bln,
	Ref (Box<VarKind>), // reference to a type
	Arr (Box<VarKind>, usize),
	Tup (EcoVec<VarKind>), // length is the number of varkinds

	// records may only differ in property names.
	// as they are part of the type itself, it's easy to compare them and cast them
	Rec (HashMap<EcoString, VarKind>),
	/*
		an example would be:
			def rec_a = rec:
				foo str,
				bar num!.
			def re_b = rec:
				foo str,
				bar num!.
			
			let something = rec_a!.
	*/

	// same principle for functions.
	// as the arguments and return type are part of the type, you can cast them
	Fun (HashMap<EcoString, VarKind>, Box<VarKind>),

	// these is the equivalent of a rust unit
	Unit,

	// resolves in the AST's second pass; if not, throws error
	Unknown,
}

/// converts lex's captures to "Token"s
pub fn tokenize(lex: &mut Lexer<TokenKind>) -> EcoVec<Token> {
	lex.clone()
		.spanned() // gives (kind, span)
		.map(|el| { // el = one (kind, span) pair
			lex.next(); // advance lexer to get the slices
			println!("token:\n{}\n----------", lex.slice()); // visualize current slice
			Token {
				kind: el.0.unwrap_or_else(|_| TokenKind::Error), // if it can't be unwrapped it's an ERROR!!
				
				literal: lex.slice().trim().into(), // literal
				
				pos: { // line and column
					let line = lex.extras.0;
					let column = lex.span().start - lex.extras.1;
					(line, column)
				}
			}
		})
		.filter(|el| el.kind != TokenKind::Comment)
		.collect()
}
