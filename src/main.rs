#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
// #![allow(clippy_pedantic)]

mod emoji;
mod parser;
mod rules;
mod ruleset;

use typed_arena::Arena;
use crate::ruleset::RuleSet;

fn main() {
    let arena = Arena::new();
    let rs = RuleSet::new(rules::get_rules(), &arena);
    let result = rs.run("fixtures/md004/md004_ko.md");

    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
