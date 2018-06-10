extern crate comrak;
extern crate typed_arena;

mod parser;
mod rules;
mod ruleset;

use ruleset::RuleCheck;

fn main() {
    let rs = ruleset::RuleSet {
        name: "Strict".to_string(),
        rules: get_rules(),
    };

    let result = rs.run("data/test1.md");
    result
        .iter()
        .for_each(|x| println!("result: {}", x.description));
}

fn get_rules() -> Vec<Box<RuleCheck>> {
    let r7 = rules::rule();
    vec![r7]
}
