extern crate comrak;
extern crate console;
extern crate typed_arena;

mod emoji;
mod parser;
mod rules;
mod ruleset;


fn main() {
    let rs = ruleset::RuleSet {
        name: "Strict".to_string(),
        rules: rules::get_rules(),
    };

    let result = rs.run("fixtures/md004/md004_ko.md");
    result.into_iter().for_each(|x| {
        println!("{}\r\n", x);
    });
}
