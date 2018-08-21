#![warn(rust_2018_idioms)]

use rusty_markdownlint::process;
use rusty_markdownlint::ruleset::RuleResult;

fn main() {
    let file = "rusty-markdownlint/fixtures/md003/md003_ko.md";
    let result = process(file);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
