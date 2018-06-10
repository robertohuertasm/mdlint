use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

#[macro_export]
macro_rules! rule {
    ($name:ident : $desc:expr; $func:expr) => {{
        pub struct $name {}

        impl RuleCheck for $name {
            fn check(&self, root: &AstNode) -> Option<RuleResult> {
                println!("Starting check: {}", $desc);
                $func($desc, root)
            }
        }
        $name {}
    }};
}

#[macro_export]
macro_rules! boxedrule {
    ($name:ident : $desc:expr; $func:expr) => {{
        Box::new(rule! {$name: $desc; $func})
    }};
}

pub mod rule1;
