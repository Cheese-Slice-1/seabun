use std::collections::HashMap;
use ecow::{EcoString, EcoVec};

use crate::def::{Token, TokenKind};

/// every possible Seabun expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	/// integer
	Num(i64),
	
	/// float
	Dot(f64),
	
	/// string literal
	Str(EcoString),
	
	/// char literal
	Chr(char),

	/// bool
	Bln(bool),

	/// variable/function/type name
	Name(EcoString),

	/// tuple literal
	Tup(EcoVec<Expr>),

	/// array literal
	Arr(EcoVec<Expr>, VarKind), // varkind = first value's varkind
	
	Rec(HashMap<EcoString, Expr>),

	/* compound expressions */
	
	// arithmetics
	Add(Box<Expr>, Box<Expr>), // +
	Sub(Box<Expr>, Box<Expr>), // -
	Mul(Box<Expr>, Box<Expr>), // *
	Div(Box<Expr>, Box<Expr>), // /
	Pow(Box<Expr>, Box<Expr>), // ^
	Mod(Box<Expr>, Box<Expr>), // %
	
	Var { // e.g. Var {id: "s".to_owned(), kind: VarKind::Str, val: Box::new(Expr::Str("a".to_owned())), ismut: false}
		id: EcoString,
		kind: VarKind,
		val: Box<Expr>,
		ismut: bool
	},
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
	Arr (Box<VarKind>, usize),
	Tup (EcoVec<VarKind>, usize),
	// records may only differ in property names.
	// as they are part of the type itself, it's easy to compare them
	Rec (HashMap<EcoString, VarKind>),
	Unknown, // resolves when making AST; if not throws error
}

/// creates a very primitive ast
pub fn primitiveast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
	let mut i: usize = 0;
	let mut res: EcoVec<Expr> = EcoVec::new();

	while i < tokens.len() {
		match tokens[i] {
			Token {kind: TokenKind::Let, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // let-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd).cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;
				res.push(parsevar(decl, false));
			},
			Token {kind: TokenKind::Var, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // var-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd).cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;
				res.push(parsevar(decl, true));
			},
			_ => {},
		}

		i += 1;
	}

	res
}

pub fn advancedast(tokebs: EcoVec<Expr>) -> EcoVec<Expr> {
	
}

// AST-RELATED FUNCTIONS

fn parsevar(tokens: EcoVec<Token>, ismut: bool) -> Expr {
	let Some((nametype, value)) = tokens
		.split_once(|tok| tok.kind == TokenKind::EqSign)
	else {
		panic!("malformed delcaration: \{insert line+column\}");
	};

	if nametype.len() > 2 {
		panic!("malformed delcaration: \{insert line+column\}");
	}

	Expr::Var {
		id: nametype
			.get(0)
			.unwrap_or_else(|_| panic!("malformed delcaration: \{insert line+column\}"))
			.literal,
		kind: match nametype.get(1) {
			Some(tok) => tovarkind(tok.literal),
			None => VarKind::Unknown,
		},
		ismut: ismut,
	}
}

pub fn tovarkind(lit: EcoString) -> VarKind {
	// predefined literals
	match &lit {
		"num" => VarKind::Num,
		"dot" => VarKind::Dot,
		"chr" => VarKind::Chr,
		"str" => VarKind::Str,
		"bln" => VarKind::Bln,
		_ => VarKind::Unknown, // non-primitive (like tuples, arrays and records)
	}
}

/*
	implementation should turn (simplified):
	[
		Let _ _
		Name "x" _
		EqSign _ _
		Num "5" _
		Plus _ _
		Num "5" _
		ExprEnd _ _
	]
	into:
	[
		Var {
			id: "x".into(),
			kind: VarKind::Unknown,
			val: Box::new(Expr::Add(
				Box::new(Expr::Num(5)),
				Box::new(Expr::Num(5)),
			)),
			ismut: false,
		},
	]
	which would then resolve to:
	[
		Expr::Var{
			id: "x".into(),
			kind: VarKind::Num,
			val: Box::new(Expr::Num(10)),
			ismut: false,
		},
	]
*/

/*
	IDEA:
	impl would slice token source by the ExprEnds (no keep)
	and LBrace and RBraces (keeping them in body)
		For example:
		[
			Token::Let,
			Token::Name("x"),
			Token::EqSign,
			Token::Num(5),
			Token::ExprEnd,
			Token::If,
			Token::Name("x"),
			Token::DblEqSign
		]
*/
