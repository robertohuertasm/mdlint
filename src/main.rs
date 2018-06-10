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

    let result = rs.run("fixtures/md001/md001.ko.md");
    // println!("{:?}", result);
    result.iter().for_each(|ref x| {
        let d = &x.details.as_ref().unwrap()[0];
        println!("result: {:?} ::> {}", x.details, x.description);
    });
}

fn get_rules() -> Vec<Box<RuleCheck>> {
    vec![rules::rule_md001()]
}
