//use std::collections::HashMap;
use std::process::exit;
use ecow::{EcoString, EcoVec};

use crate::def::{Token, TokenKind, Expr, VarKind};

/// creates a very primitive ast
pub fn primitive_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
	let mut i: usize = 0; // boring index
	let mut res: EcoVec<Expr> = EcoVec::new(); // the resulting AST (nodes)

	while i < tokens.len() {
		match &tokens[i] {
			/* DECLARATIONS */
			Token {kind: TokenKind::Let, literal, ..} | Token {kind: TokenKind::Var, literal, ..} => {
				let decl: EcoVec<Token> = tokens[i..] // let-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				
				i += decl.len() - 1;

				res.push(parse_bind(
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

			Token {kind: TokenKind::Word, literal, ..} if tokens[i+1].kind == TokenKind::EqSign => {
				
			},
			
			/* ERROR */
			Token {kind: TokenKind::Error, literal, pos, ..} => {
				println!("error: unknown token \"{literal}\": {}:{}", pos.0, pos.1);
				exit(1)
			},

			/* NOTHING */
			_ => {}
		}

		println!("{:#?}\n", res.last());

		i += 1;
	}

	res
}

pub fn advanced_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
	todo!(); // TODO: clean up AST and resolve datatypes, expressions, etc. here
}

// AST-RELATED FUNCTIONS

fn parse_bind(tokens: EcoVec<Token>, ismut: bool) -> Expr {
	//use std::any::type_name_of_val;

	// line and column of malformed/unknown token
	let errpos = tokens[0].pos;

	// "parts" are the declaration's sides
	let parts = tokens
		.get(1..tokens.len())
		.unwrap_or_else(|| {malformed("declaration", errpos);})
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
		malformed("declaration", errpos);
	}
	
	let (name, kind, value) = (
		parts
			.get(0) // parts[0] contains both name and type
			.unwrap_or_else(|| malformed("declaration", errpos))
			.get(0), // variable name; obligatory
		parts
			.get(0)
			.unwrap_or_else(|| malformed("declaration", errpos))
			.get(1), // variable type
		parts
			.get(1) // variable value
			.cloned()
		);
	
	//println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);
	
	Expr::Bind {
		id: name
			.unwrap()
			.literal
			.clone(),
		kind: match kind {
			Some(tok) => to_varkind(tok.literal.clone()),
			_ => VarKind::Unknown,
		},
		val: Box::new(parse_val(
			(*value.unwrap()).into(),
			errpos
		)),
		ismut,
	}
}

/* SIMPLE EXPRESSIONS GENERATORS */

/// precedence-based non-composite expression parser
fn parse_val(value: EcoVec<Token>, errpos: (usize, usize)) -> Expr {
	let mut res = Expr::Empty;

	// single token expression
	if value.len() < 2 {
		return match &value[0] { // kind (Expr) isn't Cow (clone-on-write)
			// a variable/type/etc. name
			Token {kind: TokenKind::Word, literal, ..} => Expr::Name(literal.clone()),

			// a integer
			Token {kind: TokenKind::Num, literal, pos} => Expr::Num (
				literal
					.parse::<i64>()
					.unwrap_or_else(|_| malformed("num literal", *pos))
			),

			// a float
			Token {kind: TokenKind::Dot, literal, pos} => Expr::Dot (
				literal
					.replace("d", ".")
					.parse::<f64>()
					.unwrap_or_else(|_| malformed("dot literal", *pos))
			),
			
			// a boolean
			Token {kind: TokenKind::Bln, literal, pos} => Expr::Bln (
				match &literal[..] {
					"true" | "yes" => true,
					"false" | "no" => false,
					_ => malformed("bln literal", *pos),
				}
			),

			// a character literal
			Token {kind: TokenKind::Chr, literal, pos} => {
				use unescaper::unescape;

				let raw = unescape(
					literal
						.get(1..literal.len()-1)
						.unwrap_or_else(|| r"\u0000")
				).unwrap_or_else(|_| malformed("chr literal", *pos));

				println!("{raw}");

				todo!();
				/*
				TODO:
					[x] convert "'_'"" to char '_'
					[ ] convert "'\_'" to char '\_'
					[ ] convert "'\uXX'" to char '\uXX'
				*/
			},

			Token {kind: TokenKind::RParen, ..} => res,
			
			_ => malformed("expression", errpos), // TODO: parse other single-token expressions
		};
	}

	let mut i: usize = 0;

	while i < value.len() {
		match value[i] {
			// a parenthesized expression
			Token {kind: TokenKind::LParen, pos, ..} => {
				let parenspan = value[i..] // sub tokens to parse
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();

				i += parenspan.len();
				
				let parenval = parse_val(
					parenspan,
					pos,
				);

				todo!(); // parse_val on everything until a ")"
			},

			Token {kind: TokenKind::Word, ref literal, ..} => res = Expr::Name(literal.clone()),
			
			_ => {
				println!("unknown token \"{}\": {}:{}", value[i].literal, value[i].pos.0, value[i].pos.1);
				exit(1);
			},
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

	todo!();

	res
}

/* UTILS */

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

fn malformed<'a>(exprkind: &'a str, pos: (usize, usize)) -> ! {
	println!("error: malformed {exprkind}: {}:{}", pos.0, pos.1);
	exit(1)
}

/*
fn unknown_tok(tok: EcoString, pos: (usize, usize)) -> ! {
	println!("error: unknown token \"{tok}\" at {}:{}", pos.0, pos.1);
	exit(1)
}
*/

// TODO: implement AST v2 using this to check if an expr is Expr::Empty
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
		Token {kind: TokenKind::Let, ..},
		Token {kind: TokenKind::Name, literal: "x".into(), ..},
		Token {kind: TokenKind::EqSign, ..}
		Token {kind: TokenKind::Num, literal: "5".into(), ..},
		Token {kind: TokenKind::Plus, ..}
		Token {kind: TokenKind::Num, literal: "5".into(), ..},
		Token {kind: TokenKind::ExprEnd, ..}
	]
	into:
	[
		Expr::Bind {
			id: "x".into(),
			kind: VarKind::Unknown,
			val: Box::new(Expr::Op {
				left: Box::new(Expr::Num(5)),
				right: Box::new(Expr::Num(5)),
				op: '+',
			}),
			ismut: false,
		},
	]
	which would then resolve as:
	[
		Expr::Bind {
			id: "x".into(),
			kind: VarKind::Num,
			val: Box::new(Expr::Num(10)),
			ismut: false,
		},
	]
*/
