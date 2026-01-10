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
	let tokens: Vec<_> = lex.clone()
		.spanned()
		.map(|el| {
			lex.next();
			(el.0 // start tuple
				.unwrap_or_else(|_| {
					let line = lex.extras.0;
					let column = lex.span().start - lex.extras.1;
					TokenKind::Error((line, column))
				}),
			el.1) // end tuple
		})
		.filter(|el| el.0 != Token::Comment)
		.collect();

	println!("{:?}", tokens);
}
