use std::collections::HashMap;
use ecow::{EcoString, EcoVec};

use crate::def::{Token, TokenKind};

/// every possible Seabun expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
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

	/* COMPUND EXPRESSIONS */

	/// a single block
	Block (EcoVec<Expr>),

	/// tuple
	Tup (EcoVec<Expr>),

	/// array
	Arr (EcoVec<Expr>, VarKind), // varkind = first value's varkind
	
	/// record
	Rec (HashMap<EcoString, Expr>),

	Fun { // a function value; in a fun name :...! {} situation a Var 
		kind: VarKind, // return type
		args: (usize, HashMap<EcoString, VarKind>), // argument types + names
		body: EcoVec<Expr>, 
	},
	
	// arithmetics
	Add (Box<Expr>, Box<Expr>), // x+y
	Sub (Box<Expr>, Box<Expr>), // x-y
	Mul (Box<Expr>, Box<Expr>), // x*y
	Div (Box<Expr>, Box<Expr>), // x/y
	Pow (Box<Expr>, Box<Expr>), // x^y
	Mod (Box<Expr>, Box<Expr>), // x%y
	
	/// represents a new push or allocation
	Var { // e.g. let s = "". -> Var {id: "s".into(), kind: VarKind::Str, val: Box::new(Expr::Str("a".to_owned())), ismut: false}
		id: EcoString,
		kind: VarKind,
		val: Box<Expr>,
		ismut: bool,
	},

	/// represents a change in an already existing variable
	ReVar { // e.g. 
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
	Arr (Box<VarKind>, usize),
	Tup (EcoVec<VarKind>, usize),

	// records may only differ in property names.
	// as they are part of the type itself, it's easy to compare them and cast them
	Rec (HashMap<EcoString, VarKind>),
	/*
		an example would be:
			def rec_a = {{
				foo str,
				bar num
			}}.
			def re_b = {{
				foo str,
				bar num
			}}.
			let something = rec_a {{}}
	*/

	// same principle for functions.
	// as the arguments and return type are part of the type, you can cast them
	Fun (HashMap<EcoString, VarKind>, Box<VarKind>),
	Unknown, // resolves when making AST; if not throws error
}

/// creates a very primitive ast
pub fn primitive_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
	let mut i: usize = 0;
	let mut res: EcoVec<Expr> = EcoVec::new();

	while i < tokens.len() {
		match tokens[i] {
			Token {kind: TokenKind::Let, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // let-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;
				res.push(parse_var(decl, false));
			},
			Token {kind: TokenKind::Var, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // var-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;
				res.push(parse_var(decl, true));
			},
			_ => { unimplemented!(); },
		}

		i += 1;
	}

	res
}

pub fn advanced_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
	todo!();
}

// AST-RELATED FUNCTIONS

fn parse_var(tokens: EcoVec<Token>, ismut: bool) -> Expr {
	let parts: EcoVec<_> = tokens
		.split(|tok| tok.kind == TokenKind::EqSign)
		.collect::<EcoVec<_>>();
	
	println!("{:#?}", parts);
	todo!();
	/*
	let (name, kind, value) = (
		parts
			.get(0)
			.expect("malformed edclaration")
			.get(0), // variable name; obligatory
		parts
			.get(0)
			.get(1), // variable type
		parts
			.get(1) // variable value
		);

	if parts[0].len() > 2 {
		panic!("malformed delcaration: {{insert line+column}}");
	}

	Expr::Var {
		id: name
			.unwrap()
			.literal
			.clone(),
		kind: match kind {
			Some(tok) => to_varkind(tok.literal),
			None => VarKind::Unknown,
		},
		val: Box::new(parse_val(value)),
		ismut,
	}
	*/
}

/// precedence-based parsing
fn parse_val(content: EcoVec<Token>) -> Expr {
	if content.len() < 2 {
		return match &content[0].kind {
			TokenKind::Word => Expr::Name(content[0].literal.clone()),
			_ => unimplemented!(),
		};
	}

	todo!();
}

pub fn to_varkind(lit: EcoString) -> VarKind {
	// predefined literals
	match &lit[..] {
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
