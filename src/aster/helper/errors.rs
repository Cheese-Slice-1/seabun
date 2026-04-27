use crate::def::{TokenKind, CodePos};
use std::process::exit;

extern crate ecow;
use ecow::EcoString;

/// exists the program with an error about a malformed something, where
/// what the something is is provided by the caller (e.g. "chr literal")
#[allow(unused)]
#[inline]
pub fn malformed(exprkind: &str, pos: CodePos) -> ! {
    println!("error: malformed {exprkind}: {pos}");
    exit(1)
}

#[allow(unused)]
#[inline]
pub fn unimp() -> ! {
    println!("yeah, uh, sooooo, i haven't implemented this :b");
    exit(1)
}

#[allow(unused)]
#[inline]
pub fn stop_here() -> ! {
    println!("testing smth, stopping exec!!");
    exit(2)
}

#[allow(unused)]
#[inline]
pub fn dumbass_compiler(whatever: TokenKind, literal: &EcoString) -> ! {
    println!("that is not a {whatever:?}, you mf dumbass compiler: {literal}");
    exit(3)
}

pub fn unknown(tok: &EcoString, pos: CodePos) -> ! {
    println!("error: unknown token \"{tok}\" at {pos}");
    exit(1)
}
