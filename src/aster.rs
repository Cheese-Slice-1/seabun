use std::collections::HashMap;

#[path="def.rs"]
mod def;
use def::{Token};

/// every possible Seabun expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
	/// integer
	Num(i64),
	
	/// float
	Dot(f64),
	
	/// string literal
	Str(String),
	
	/// char literal
	Chr(char),

	/// bool
	Bln(bool),

	/// variable/function/type name
	Name(String),

	/// tuple literal
	Tup(Vec<Expr>),

	/// array literal
	Arr(Vec<Expr>, VarKind), // varkind = first value's varkind
	
	Rec(HashMap<String, Expr>),

	/* compound expressions */
	
	// arithmetics
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Pow(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	
	// variable declarations
	VarHalf { // e.g. VarHalf{id: "s".to_owned(), kind: VarKind::Str, ismut: false}
		id: String,
		kind: VarKind,
		ismut: bool
	},
	VarFull { // e.g. VarFull{id: "s".to_owned(), kind: VarKind::Str, val: Box::new(Expr::Str("a".to_owned())), ismut: false}
		id: String,
		kind: VarKind,
		val: Box<Expr>,
		ismut: bool
	},
}

fn primitiveast(tokens: Vec<Token>) -> Vec<Expr> {
	
}

/*
	implementation should turn:
	[
		Token::Let,
		Token::Name("x".to_owned()),
		Token::EqSign,
		Token::Num(5),
		Token::Plus,
		Token::Num(5),
		Token::ExprEnd,
	]
	into:
	[
		Expr::VarFull(
			"x".to_owned(),
			VarKind::Unknown,
			Box::new(Expr::Add(
				Box::new(Expr::Num(5)),
				Box::new(Expr::Num(5)),
			)),
			IMMUTABLE,
		),
	]
	which would then resolve to:
	[
		Expr::LetFull(
			"x".to_owned(),
			VarKind::Num,
			Box::new(Expr::Num(10)),
			IMMUTABLE,
		),
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
