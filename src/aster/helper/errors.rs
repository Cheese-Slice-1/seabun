use crate::def::{CodePos, TokenKind};
use std::process::exit;

extern crate ecow;
use ecow::EcoString;

/// exists the program with an error about a malformed something, where
/// what the something is is provided by the caller (e.g. "chr literal")
#[allow(unused)]
#[inline]
pub fn malformed(exprkind: &str, pos: CodePos) -> ! {
    eprintln!("error: malformed {exprkind}: {pos}");
    exit(1)
}

#[allow(unused)]
#[inline]
pub fn unimp(pos: CodePos) -> ! {
    eprintln!("yeah, uh, sooooo, i haven't implemented this :b ({pos})");
    exit(1)
}

#[allow(unused)]
#[inline]
pub fn stop_here(msg: &str) -> ! {
    eprintln!("testing smth, stopping exec!!\n{msg}");
    exit(2)
}

#[allow(unused)]
#[inline]
pub fn dumbass_compiler(whatever: TokenKind, literal: &EcoString) -> ! {
    eprintln!("that is not a {whatever:?}, you mf dumbass compiler: {literal}");
    exit(3)
}

pub fn unknown(tok: &EcoString, pos: CodePos) -> ! {
    eprintln!("error: unknown token {tok} at {pos}");
    exit(1)
}

