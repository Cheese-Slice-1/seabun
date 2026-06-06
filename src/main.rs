use std::{env::args, fs::read_to_string};
use ecow::{EcoString, EcoVec};

mod def;
mod aster;
mod codegen;

use def::{TokenKind, tokenize};
use aster::build::make_ast;

use logos::Logos;

fn main() {
    let fname = args().nth(1).unwrap_or_else(|| {
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
    
    let prim_ast = make_ast(tokens_spanned);
    
    println!("{:#?}", prim_ast)
}
