#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use rusty_markdownlint::process;
use rusty_markdownlint::ruleset::RuleResult;

fn main() {
    let file = "rusty-markdownlint/fixtures/md009/md009_ko_single_line.md";
    let result = process(file);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
