extern crate comrak;
extern crate console;
extern crate typed_arena;

mod emoji;
mod parser;
mod rules;
mod ruleset;

use ruleset::RuleCheck;

fn main() {
    let rs = ruleset::RuleSet {
        name: "Strict".to_string(),
        rules: get_rules(),
    };

    let result = rs.run("fixtures/md001/md001.ko.md");
    // println!("{:?}", result);
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}

fn get_rules() -> Vec<Box<RuleCheck>> {
    vec![rules::rule_md001()]
}
