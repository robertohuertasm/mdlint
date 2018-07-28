use comrak::nodes::AstNode;
use crate::ruleset::{RuleCheck, RuleResult};

#[macro_use]
mod macros;
mod extensions;
mod md001;
mod md002;
mod md003;
mod md004;

crate fn get_rules() -> Vec<Box<dyn RuleCheck>> {
    vec![
        boxedrule!{ MD001: md001::check },
        boxedrule!{ MD002: md002::check },
        boxedrule!{ MD003: md003::check },
        boxedrule!{ MD004: md004::check },
    ]
}
