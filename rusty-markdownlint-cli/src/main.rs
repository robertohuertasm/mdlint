#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]
// #![allow(clippy_pedantic)]

use rusty_markdownlint::process;
use rusty_markdownlint::ruleset::RuleResult;

fn main() {
    let file = "rusty-markdownlint/fixtures/md004/md004_ko.md";
    let result = process(file);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
