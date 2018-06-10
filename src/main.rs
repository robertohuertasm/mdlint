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
    result.iter().for_each(|x| println!("{}", x.description));
}

fn get_rules() -> Vec<Box<RuleCheck>> {
    let r4 = rules::get4();
    let r5 = rules::get5();
    let r6 = rules::get6();
    vec![Box::new(r4), Box::new(r5), r6]
}
