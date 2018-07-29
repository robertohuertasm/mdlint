#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

mod emoji;
mod parser;
mod rules;
pub mod ruleset;

use crate::ruleset::{RuleResult, RuleSet};
use typed_arena::Arena;

pub fn process(path: &str) -> Vec<RuleResult> {
    let arena = Arena::new();
    let rs = RuleSet::new(rules::get_rules(), &arena);
    rs.run(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::process("fixtures/md004/md004_ko.md");
        assert!(!result.is_empty());
    }
}
