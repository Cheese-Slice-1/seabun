use std::{env, fs};

#[path="def.rs"]
mod def;
use def::{*};

use logos::Logos;

fn main() {
	let fname = env::args()
		.nth(1)
		.expect("please, provide a filename");

	let src: String = fs::read_to_string(fname)
		.expect("couldn't find the provided file");

	let mut lex = TokenKind::lexer(src.as_str());
	let tokens_spanned: Vec<_> = tokenize(&mut lex);

	println!("{:#?}", tokens_spanned);
}

/*
	Let -> consume and start Expr::Var
		Name -> evaluate (name ->)
		Name? (kind) -> evaluate (name -> varkind)
			<- cut (before eqsign)
		EqSign -> consume
			<- cut (after eqsign)
		Expr -> evaluate 
			<- cut (before exprend)
		ExprEnd -> consume
			<- cut (after exprend; delete if next is empty)
	...
*/
