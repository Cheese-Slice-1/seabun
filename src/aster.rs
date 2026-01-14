use std::collections::HashMap;
use ecow::{EcoString, EcoVec};

use crate::def::{Token, TokenKind};

/// every possible Seabun expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
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
		operator: char,
	},

	/* COMPLEX COMPOUND EXPRESSIONS */
	/* they produce an Expr::Empty value and a VarType::Unknown kind (tl;dr: can't be values) */
	
	/// represents a new push or allocation
	Decl { // e.g. let s = "". -> Decl {id: "s".into(), kind: VarKind::Str, val: Box::new(Expr::Str("a".to_owned())), ismut: false}
		id: EcoString,
		kind: VarKind,
		val: Box<Expr>,
		ismut: bool,
	},

	/// represents a change in an already existing variable
	ReDecl { // e.g. 
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
	Tup (EcoVec<VarKind>), // length is the number of varkinds

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

	// these is the equivalent of a rust unit
	Unit,

	// resolves in the AST's second pass; if not, throws error
	Unknown,
}

/// creates a very primitive ast
pub fn primitive_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
	let mut i: usize = 0; // boring index
	let mut res: EcoVec<Expr> = EcoVec::new(); // the resulting AST (nodes)

	while i < tokens.len() {
		match tokens[i] {
			/* DECLARATIONS */
			Token {kind: TokenKind::Let, ref literal, ..} | Token {kind: TokenKind::Var, ref literal, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // let-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;

				res.push(parse_decl(
					decl, // tokens that conform th declaration

					// always "let" or "var". nothing else should be possible
					// if there's something else blame it on me or the lexer
					// cuz lil bro shouldn't be doing that...
					match &literal[..] {
						"let" => false,	// ismut = false
						"var" => true,	// ismut = true
						_ => panic!("this shouldn't happen!!"),
					},
				));
			},
			
			/* ERROR */
			Token {kind: TokenKind::Error(linecol), literal, ..} => {
				unknown_tok(literal.clone, linecol);
			},

			/* NOTHING */
			_ => {}
		}

		i += 1;
	}

	res
}

pub fn advanced_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
	todo!();
}

// AST-RELATED FUNCTIONS

fn parse_decl(tokens: EcoVec<Token>, ismut: bool) -> Expr {
	//use std::any::type_name_of_val;

	// span the error occupies (no shit sherlock)
	let errspan = (tokens[0].span.0, tokens[tokens.len()-1].span.1);

	// "parts" are the declaration's sides
	let parts = tokens
		.get(1..tokens.len())
		.unwrap_or_else(|| {malformed("declaration", errspan);})
		.split(|tok| tok.kind == TokenKind::EqSign)
		.collect::<EcoVec<_>>();
	
	/*
	for part in parts {
		println!("contains:\n{:#?}\n", part);
		println!("is type:\n{:?}", std::any::type_name_of_val(&part));
		println!();
	}
	*/

	if parts.get(0).unwrap().len() > 2 || parts.get(0).unwrap().is_empty() {
		malformed("declaration", errspan);
	}
	
	let (name, kind, value) = (
		parts
			.get(0) // parts[0] contains both name and type
			.unwrap_or_else(|| malformed("declaration", errspan))
			.get(0), // variable name; obligatory
		parts
			.get(0)
			.unwrap_or_else(|| malformed("declaration", errspan))
			.get(1), // variable type
		parts
			.get(1) // variable value
			.cloned()
		);
	
	//println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);
	
	Expr::Decl {
		id: name
			.unwrap()
			.literal
			.clone(),
		kind: match kind {
			Some(tok) => to_varkind(tok.literal.clone()),
			None => VarKind::Unknown,
		},
		val: Box::new(parse_val(
			(*value.unwrap()).into(),
			errspan
		)),
		ismut,
	}
}

/// precedence-based non-composite expression parser
fn parse_val(value: EcoVec<Token>, errspan: (usize, usize)) -> Expr {
	let mut ret = Expr::Empty;

	if value.len() < 2 {
		return match value.clone()[0].kind { // kind (Expr) isn't Cow (clone-on-write)
			// a single integer
			TokenKind::Word => Expr::Name(value[0].literal.clone()),
			TokenKind::Num => Expr::Num (
				value[0]
					.literal
					.parse::<i64>()
					.unwrap_or_else(|_| malformed("declaration", errspan))
			),

			// a single float
			TokenKind::Dot => Expr::Dot (
				value[0]
					.literal
					.replace("d", ".")
					.parse::<f64>()
					.unwrap_or_else(|_| malformed("declaration", errspan))
			),

			_ => unimplemented!(),
		};
	}

	let mut i: usize = 0;

	/*
		ideas:
			- parse tokens as if they were characters
			- no operator precedence; pure LtR with precedence for parentheses
			- LParen => recursive parse_eval
			- RParen => forcibly return from parse_eval
	*/

	todo!();

	ret
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

fn malformed<'a>(exprkind: &'a str, span: (usize, usize)) -> ! {
	println!("malformed {exprkind}: {:?}", span.0..span.1);
	std::process::exit(1)
}

fn unknown_tok(tok: EcoString, span: (usize, usize)) -> ! {
	println!("unknown token \"{tok}\" at line {:?}", span.0..span.1);
	std::process::exit(1)
}

fn check(expr: Expr) -> Result<Expr, ()> {
	if expr == Expr::Empty {
		println!("");
		return Err(());
	}

	Ok(expr)
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
