// TODO: WHY DOESN'T IT FUCKING WORK I'M GONNA CRY WHATTTTTTTTTTTT (QnQc)
// someone help I BEG OF YOU :""/

//use std::collections::HashMap;
use std::process::exit;
use ecow::{EcoString, EcoVec/*, eco_vec*/};
use unescaper::unescape;

use crate::def::{Token, TokenKind, Expr, VarKind, BindKind};

// TODO: defined ids; move it to where it belongs!!
//static mut BINDINGS: EcoVec<(String, Expr)> = eco_vec![];

/// creates a very primitive ast
pub fn primitive_ast(tokens: EcoVec<Token>) -> EcoVec<Expr> {
	let mut i: usize = 0; // boring index
	let mut res: EcoVec<Expr> = EcoVec::new(); // the resulting AST (nodes)

	while i < tokens.len() {
		match &tokens[i] {
			/* DECLARATIONS */
			Token {kind: TokenKind::Let, literal, ..} |
			Token {kind: TokenKind::Mut, literal, ..} |
			Token {kind: TokenKind::Def, literal, ..}
			=> {
				let bind: EcoVec<Token> = tokens[i..] // let-related chunk
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				i += bind.len() - 1;

				let bind_params = match &literal[..] { // NOTE: don't use kind. do not. please.
					"mut" => (true, false),		// mutable
					"let" => (false, false),	// mutablen't (badum tsss)
					"def" => (false, true),		// mutablen't and a "typedef" (b a d u m   t s s s)
					_ => panic!("this shouldn't happen!! wtf!!!!"),
				};

				res.push(parse_bind(
					bind, // tokens that conform th declaration

					// always Let or Mut. nothing else should be possible
					// if there's something else blame it on me or the lexer
					// cuz lil bro shouldn't be doing that...
					bind_params
				));
			},

			Token {kind: TokenKind::Word, literal, ..} if tokens[i+1].kind == TokenKind::EqSign => {
				let rebind: EcoVec<Token> = tokens[i..]
					.iter()
					.take_while(|tok| tok.kind != TokenKind::ExprEnd)
					.cloned()
					.collect::<EcoVec<Token>>();
				i += rebind.len() - 1;

				res.push(parse_rebind(
					rebind,
				));
			},
			
			/* ERROR */
			Token {kind: TokenKind::Error, literal, pos, ..} => {
				println!("error: unknown token \"{literal}\": {}:{}", pos.0, pos.1);
				exit(1)
			},

			/* NOTHING */
			_ => {}
		}

		//println!("{:#?}\n", res.last());

		i += 1;
	}

	res
}

/*
pub fn advanced_ast(_ast: EcoVec<Expr>) -> EcoVec<Expr> {
	unimplemented!(); // TODO: clean up AST and resolve datatypes, expressions, etc. here
}
*/

// AST-RELATED FUNCTIONS

/// parses a binding
/// takes two parameters:
/// - "tokens": the vector of tokens that conform the bind
/// - "bind_kind": the chosen bind type as a bool pair representing (IS_MUT, IS_DEF)
fn parse_bind(tokens: EcoVec<Token>, bind_kind: (bool, bool)) -> Expr {
	//use std::any::type_name_of_val;

	// line and column of malformed/unknown token
	let errpos = tokens[0].pos;

	// "parts" are the declaration's sides
	let parts = tokens
		.get(1..tokens.len())
		.unwrap_or_else(|| malformed("declaration", errpos))
		.split(|tok| tok.kind == TokenKind::EqSign)
		.collect::<EcoVec<_>>();
	
	/*
	for part in parts {
		println!("contains:\n{:#?}\n", part);
		println!("is type:\n{:?}", std::any::type_name_of_val(&part));
		println!();
	}
	*/

	// if there's nothing between let/mut/def and "=", scream "malformed declaration"
	if parts.get(0)
		.unwrap_or_else(|| malformed("declaration", errpos))
		.is_empty()
	{
		malformed("declaration", errpos);
	}
	
	let (name, var_kind, value) = (
		parts
			.get(0) // parts[0] contains both name and type
			.unwrap_or_else(|| malformed("declaration", errpos))
			.get(0), // variable name; obligatory
		parts
			.get(0)
			.unwrap_or_else(|| malformed("declaration", errpos))
			.get(1..), // variable type
		parts
			.get(1) // variable value
			.cloned()
	);
	
	let holds = match var_kind {
		Some(toks) => to_varkind(EcoVec::from(toks), errpos),
		_ => VarKind::Unknown,
	};

	//println!("{:?}\n{:?}\n{:#?}\n", &name, &kind, &value);
	
	Expr::Bind {
		id: name
			.unwrap()
			.literal
			.clone(),
		kind: match bind_kind {
			(true, _) => BindKind::MutValue(holds),
			(false, false) => BindKind::Value(holds), //
			(false, true) => BindKind::Define(holds)
		},
		val: Box::new(parse_val(
			(*value.unwrap_or_else(|| malformed("declaration", errpos))).into(),
			errpos,
			false
		)),
	}
}

fn parse_rebind(_tokens: EcoVec<Token>) -> Expr {
	unimp()
}

/* SIMPLE EXPRESSIONS GENERATORS */

/// precedence-based non-composite expression parser
fn parse_val(value: EcoVec<Token>, errpos: (usize, usize), isnested: bool) -> Expr {
	// expression to return
	let mut res = Expr::Empty;

	// single token expression
	if value.len() == 1 {
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
					"yes" | "true" => true,
					"no" | "false" => false,
					_ => malformed("bln literal", *pos),
				}
			),

			// a character literal
			Token {kind: TokenKind::Chr, literal, pos} => {
				let raw: char =
					unescape(
						literal
							.get(1..literal.len()-1)
							.unwrap_or(r"\u0000")
					)
					.unwrap_or_else(|_| malformed("chr literal", *pos))
					.chars()
					.nth(0)
					.unwrap_or('\x00');

				Expr::Chr(raw)
			},

			Token {kind: TokenKind::Str, literal, pos, ..} => {
				let unescaped = EcoString::from(
					unescape(
						literal
							.get(1..literal.len()-1)
							.unwrap_or("")
					)
					.unwrap_or_else(|_| malformed("str literal", *pos))
				);

				Expr::Str(unescaped)
			},

			Token {kind: TokenKind::RParen, ..} => if isnested { res } else { malformed("expression", errpos) },
			
			_ => malformed("or unimplemented expression", errpos), // TODO: parse other single-token expressions
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
					true
				);
				
				res = parenval;
				// parse_val on everything until a ")"
			},

			Token {kind: TokenKind::RParen, pos, ..} => {
				if isnested {
					return res;
				} else {
					malformed("parenthesized expression", pos);
				}
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

	//unimplemented!();

	res
}

/* UTILS */

pub fn to_varkind(toks: EcoVec<Token>, pos: (usize, usize)) -> VarKind {
	// predefined literals
	if toks.len() < 2 {
		let Some(tok) = toks.get(0) else { return VarKind::Unknown; };
		return match &tok.literal[..] {
			"num" => VarKind::Num,
			"dot" => VarKind::Dot,
			"chr" => VarKind::Chr,
			"str" => VarKind::Str,
			"bln" => VarKind::Bln,
			_ => VarKind::Unknown, // non-primitive (like custom types, tuples, arrays and records)
		};
	}
	
	// TODO: implement complex types like funs, recs, arrays, etc.
	// very important so remember eh?
	unimp();
}

/// exists the program with an error about a malformed something, where
/// what the something is is provided by the caller (e.g.: chr literal)
fn malformed<'a>(exprkind: &'a str, pos: (usize, usize)) -> ! {
	println!("error: malformed {exprkind}: {}:{}", pos.0, pos.1);
	exit(1)
}

fn unimp() -> ! {
	println!("yeah, uh, sooooo, i haven't implemented this :b");
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
			is_mut: false,
		},
	]
	which would then resolve as:
	[
		Expr::Bind {
			id: "x".into(),
			kind: VarKind::Num,
			val: Box::new(Expr::Num(10)),
			is_mut: false,
		},
	]
*/
