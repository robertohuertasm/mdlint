use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult, RuleResultDetails};

mod md001;
mod md002;

#[macro_export]
macro_rules! rule {
    ($name:ident : $func:expr) => {{
        pub struct $name {}

        impl RuleCheck for $name {
            fn check<'a>(&self, root: &'a AstNode<'a>) -> RuleResult {
                $func(root)
            }
        }
        $name {}
    }};
}

#[macro_export]
macro_rules! boxedrule {
    ($name:ident : $func:expr) => {{
        Box::new(rule! {$name: $func})
    }};
}

pub trait VecExt {
    fn to_option(self) -> Option<Vec<RuleResultDetails>>;
}

impl VecExt for Vec<RuleResultDetails> {
    fn to_option(self) -> Option<Vec<RuleResultDetails>> {
        if self.len() > 0 {
            Some(self)
        } else {
            None
        }
    }
}

pub fn get_rules() -> Vec<Box<RuleCheck>> {
    vec![
        boxedrule!{ MD001: md001::check },
        boxedrule!{ MD002: md002::check },
    ]
}
