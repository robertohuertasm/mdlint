#![warn(rust_2018_idioms)]

use mdlint::{all, process, RuleResult};

fn main() {
    let file = "mdlint/fixtures/md011/md011_ko.md";
    let rules = Some(all());
    let result = process(file, rules);
    print(result);
}

fn print(result: Vec<RuleResult>) {
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
