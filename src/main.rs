use std::{env::args, fs::read_to_string};
use ecow::{EcoString, EcoVec};

mod def;
use def::{tokenize, TokenKind};

mod aster;
use aster::{primitive_ast};

use logos::Logos;

fn main() {
	let fname = args()
		.nth(1)
		.unwrap_or_else(|| {
			println!("usage: seabun FILENAME.cbun (-o EXECUTABLENAME)");
			std::process::exit(0);
		});

	let src: EcoString = read_to_string(fname.clone())
		.unwrap_or_else(|_| {
			println!("couldn't find provided file: {fname}");
			std::process::exit(1);
		})
		.into();

	let mut lex = TokenKind::lexer(src.as_str());
	let tokens_spanned: EcoVec<_> = tokenize(&mut lex);

	//println!("{:#?}", tokens_spanned);

	let prim_ast = primitive_ast(tokens_spanned);

	println!("{:#?}", prim_ast)
}

/*
	Let -> consume and start Expr::Decl
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
