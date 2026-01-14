use std::{env::args, fs::read_to_string};
use ecow::{EcoString, EcoVec};

mod def;
use def::{*};

mod aster;
use aster::{Expr, VarKind, primitive_ast};

use logos::Logos;

fn main() {
	let fname = args()
		.nth(1)
		.expect("please, provide a filename");

	let src: EcoString = read_to_string(fname.clone())
		.unwrap_or_else(|_| {
			panic!("couldn't find provided file {fname}")
		})
		.into();

	let mut lex = TokenKind::lexer(src.as_str());
	let tokens_spanned: EcoVec<_> = tokenize(&mut lex);

	//println!("{:#?}", tokens_spanned);

	let prim_ast = primitive_ast(tokens_spanned);
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
