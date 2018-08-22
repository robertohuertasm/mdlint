#![warn(rust_2018_idioms)]

use rusty_markdownlint::{all, process, RuleResult};

fn main() {
    let file = "rusty-markdownlint/fixtures/md010/md010_ko.md";
    let rules = Some(all());
    let result = process(file, rules);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
