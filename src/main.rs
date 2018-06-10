extern crate comrak;
extern crate typed_arena;

mod ruleset;
mod rules;
mod parser;

use ruleset::RuleCheck;

fn main() {
    let rs = ruleset::RuleSet {
        name: "Strict".to_string(),
        rules: get_rules()
    };
    
    let result = rs.run("data/test1.md");
    result.iter().for_each(|x| println!("{}", x.description));
}

fn get_rules() -> Vec<Box<RuleCheck>> {
    let r1 = rules::get1();
    let r2 = rules::get2();
    let r3 = rules::get3();
    let r4 = rules::get4();
    let r5 = rules::get5();
    vec![Box::new(r1), Box::new(r2), Box::new(r3), Box::new(r4), Box::new(r5)]
}
