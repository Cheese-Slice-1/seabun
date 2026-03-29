use std::collections::HashMap;
use ecow::{EcoString, EcoVec};

extern crate logos;

use logos::{Lexer, Logos, Skip};

// use chumsky::prelude::*;

/// a seabun token; contains all necessary information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub kind: TokenKind, // what it is
	pub literal: EcoString, // raw form
	pub pos: CodePos, // exact position
}

// a position in code
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodePos (pub usize, pub usize);

/*
	PRIORIDADES:
	0. comentarios
	1. simbolos
	2. literales
	3. palabras clave
	4. IDs y tipos
*/
#[derive(Logos)]
#[derive(Clone, Debug, PartialEq, Eq)]
#[logos(extras=(usize, usize))]
#[logos(skip (r#"\s+?"#, |l| {
	for c in l.slice().chars() {
		if c == '\n' {
			l.extras.0 += 1;
			l.extras.1 = l.span().end;
		}
	}
	Skip
}))]
#[repr(u8)]
pub enum TokenKind {
	/* SYMBOL-BASED TOKENS */

	#[regex(r";[^\x00-\x1F]*", priority=110)]
	Comment, // #a line comment
	
	#[regex(r";[*][^\x00-\x1F]+?", priority=120)]
	Doc,

	#[regex("[.]", priority=100)]
	ExprEnd, // end of an expression

	#[regex(",", priority=100)]
	Comma, // separates expressions

	#[regex("=", priority=100)]
	EqSign, // usu. used for assigning bindings

	#[regex(":", priority=100)]
	Colon, // separates name from parameter/argument list
	
	#[regex("!", priority=100)]
	Bang, // ends parameter/argument list

	#[regex("[(]", priority=100)]
	LParen, // nested expr start
	
	#[regex("[)]", priority=100)]
	RParen, // nested expr end

	#[regex(r"\[", priority=100)]
	LBracket, // array start
	
	#[regex(r"\]", priority=100)]
	Rbracket, // array end

	#[regex("[{]", priority=100)]
	LBrace, // block start

	#[regex("[}]", priority=100)]
	RBrace, // block end

	#[regex("[{][{]", priority=101)]
	LDblBrace, // tuple literal start

	#[regex("[}][}]", priority=101)]
	RDblBrace, // tuple literal end

	#[regex("[+]", priority=100)]
	Plus,

	#[regex("-", priority=100)]
	Minus,

	#[regex("[*]", priority=100)]
	Star, // multiplication

	#[regex("/", priority=100)]
	Slash, // division (x/y)

	#[regex(r"\^", priority=100)]
	Caret, // power (x^y)

	#[regex("%", priority=100)]
	Percent, // modulo (x%y)

	#[regex("#", priority=100)]
	Hash, // type-casts like "as"
	
	#[regex("@", priority=100)]
	AtSign, // like C "*T"; pointer type (@T) or "value at address" (bc it's read at lololol)

	#[regex("~", priority=100)]
	Tilde, // "address of"; a tilde because & is bitwise and

	#[regex("->", priority=100)]
	Arrow, // will denote return type for funs and mabe smth more??

	#[regex("¬", priority=100)]
	NotSign, // ¬a

	#[regex("&", priority=100)]
	Ampersand, // a & b

	#[regex(r"\|", priority=100)]
	Pipe, // a | b

	#[regex(r"\\", priority=100)]
	BackSlash, // a \ b

	/* KEYWORS-BASED TOKENS */

	#[regex("show", priority=60)]
	Show, // print an array of chr-compatible elements to screen (no newline)

	#[regex("read", priority=60)]
	Read, // read a str from stdin

	#[regex("as", priority=60)]
	As, // type casting
	
	#[regex("let", priority=60)]
	Let, // immutable binding declaration

	#[regex("mut", priority=60)]
	Mut, // mutable binding declaration

	#[regex("def", priority=60)]
	Def, // type definition
	
	#[regex("fun", priority=60)]
	Fun, // function literal; fun: parameter type, ... -> type

	#[regex("give|back", priority=60)]
	Return,

	#[regex("rec", priority=60)]
	Rec, // record literal; rec: field type, ...!
	
	#[regex("if", priority=60)]
	If,
	
	#[regex("elif", priority=60)]
	Elif,
	
	#[regex("else", priority=60)]
	Else,

	#[regex("loop", priority=60)]
	Loop, // rust-style infinite loop

	#[regex("while", priority=60)]
	While,

	#[regex("every", priority=60)]
	Every, // every x, [1, 2, 3] ...; basically a for-in or for-of loop

	#[regex("do", priority=60)]
	Do,
	
	/* NON-KEYWORD-BASED TOKENS */
	
	#[regex(r"\d+", priority=40)]
	Num, // 1, 2, 3, 4; can be narrowed down to any num, unum, or chr type
	
	#[regex(r"[\d]+d[\d]*", priority=40)]
	Dot, // 1d5, 0d103, -9d9; no "d.." because it'd be a type id
	
	#[regex(r#"("")|(")([^"\\\x00-\x1F]|\\(["\\bnfrt]|u[a-fA-F0-9]{4}|x[a-fA-F0-9]{2}))+?(")"#, priority=40)]
	Str, // "hola", "HOLA", "HoLa123", "\""

	#[regex(r#"(<{2}>{2})|(<{2})([^\x00-\x1F]|\\>)+?(>{2})"#, priority=40)]
	RawStr, // <<hola>>, <<HOLA>>, <<it's "fine"!>>, <<only escape is\>>> (only escape is >)
	// i hope the quirky (raw) string doesn't get interpreted as a separate string inside a normal string hwlp-
	// (or viceversa lmao)
	
	// ONE character or escape
	#[regex(r#"''|'([^'\\\x00-\x1F]|\\(['\\bnfrt]|u[a-fA-F0-9]{4}|x[a-fA-F0-9]{2}))'"#, priority=40)]
	Chr, // 'c', '\u6F', '\\', '\'', '\n'
	
	#[regex("true|false|yes|no", priority=40)]
	Bln, // true, false

	#[regex(r#"[\w][\w\d]*"#, priority=20)]
	Word, // foo, bar_, _baz, bar2, seabun
	
	/* ERROR TYPE REPRESENTING (line, column) */
	
	Error,
}

/// every possible Seabun expression
#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	/* SIMPLE EXPRESSIONS */
	// these consist of a single non-self-referential value

	/// empty expression; throws an error in declarations
	Empty,

	/// signed integer
	Num (isize),

	/// unsigned integer
	Pos (usize),
	
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

	/// tuple literal. e.g. {{1, 2}}; {{'a', 62, }}
	Tup (EcoVec<Expr>),

	/// array
	Arr (EcoVec<Expr>, VarKind), // varkind = first value's varkind
	
	/// record literal. e.g rec: x num!;
	Rec (HashMap<EcoString, (Expr, VarKind)>), // TODO: rethink this type of expression asap

	/// function literal. e.g. fun: arg num {}; fun -> chr {}; fun: arg num -> chr {}
	Fun { 
		kind: VarKind, // return type
		args: (usize, HashMap<EcoString, VarKind>), // argument types + names
		body: Box<Expr>, 
	},
	
	/// arithmetic operation
	MathOp { //x+y, x-y, x*y, x/y, x^x, x%x
		left: Box<Expr>,
		right:Box<Expr>,
		op: char,
	},

	/* COMPLEX COMPOUND EXPRESSIONS */
	/* they produce an Expr::Empty value and a VarType::Unknown kind (tl;dr: can't be values) */
	
	/// represents a new push to the stack, allocation on the heap, or type definition
	Bind { // e.g. let s = "". -> Bind {id: "s".into(), kind: (BindKind::Value, VarKind::Str), val: Box::new(Expr::Str("a".into()))}
		id: EcoString,
		kind: (BindKind, VarKind),
		val: Box<Expr>,
	},

	/// represents a change to an already existing mutable binding
	ReBind { // e.g. a = 2. -> ReBind {id: "a".into(), val: Box::new(Expr::Num(2))}
		id: EcoString,
		val: Box<Expr>,
	}
}

/// every possible variable kind in Seabun;
/// these only contain essential info
#[derive(Clone, Debug, PartialEq)]
pub enum VarKind {
	Num, // = isize
	Unum, // = isize
	Dot, // like fsize
	Str, // = [u8]
	Chr, // = u8
	Bln, // just bool
	NumX (NBit), // nXX
	UnumX (NBit), // nXX
	DotX (NBit),
	ChrX (NBit), // UTF-X
	Ref (Box<VarKind>), // reference to a type
	Arr (Box<VarKind>, usize), // the usize is the number of elements of the array
	Tup (EcoVec<VarKind>),

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

	// these is the equivalent of a rust unit; internally 0(?)
	Unit,

	// resolves in the AST's second pass; if not, throws error
	Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBit {
	T8,
	T16,
	T32,
	T64,
	T128,
	TSize
}

// TODO: use this instead of "is_mut"
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindKind {
	Value,
	MutValue,
	Define,
}

/// converts lex's captures to "Token"s
pub fn tokenize(lex: &mut Lexer<TokenKind>) -> EcoVec<Token> {
	lex.clone()
		.spanned() // gives (kind, span)
		.map(|el| { // el = one (kind, span) pair
			lex.next(); // advance lexer to get the slices

			//println!("token:\n{}\n----------", lex.slice()); // visualize current slice

			Token {
				kind: el.0.unwrap_or_else(|_| TokenKind::Error), // if it can't be unwrapped it's an ERROR!!
				
				literal: lex.slice().trim().into(), // literal
				
				pos: { // line and column
					let line = lex.extras.0 + 1;
					let column = lex.span().start - lex.extras.1;
					CodePos(line, column)
				}
			}
		})
		.filter(|el| el.kind != TokenKind::Comment)
		.collect()
}

impl Expr {
	// TODO: implement AST v2 using this to check if an expr is Expr::Empty
	#[inline]
	fn check(expr: &Self) -> bool {
		*expr == Expr::Empty
	}
}

#[macro_export]
#[allow(unused)]
macro_rules! get_t {
	($from:expr) => {
		EcoString::from(std::any::type_name_of_val($from))
	};

	($($from:expr),+) => { {
			let origin = [$($from),+];
			let mut res = EcoVec::<EcoString>::with_capacity(origin.len());
			for val in origin.iter() {
				res.push(std::any::type_name_of_val(val).into());
			}
			res
		}
	};
}

/// array holding all types of token that throw an error when a
/// literal precedes them (e.g. 1 + 1 => "1" is the prefix of "+")
/// (dirty ass solution but imo easy to use, sorry for your eyes :P)
#[allow(unused)]
const NO_VAL_PREFIX: [TokenKind; 9] = [
	TokenKind::LParen,
	TokenKind::LBracket,
	TokenKind::LDblBrace,
	TokenKind::Num,
	TokenKind::Dot,
	TokenKind::Chr,
	TokenKind::Str,
	TokenKind::Fun,
	TokenKind::Word,
];
