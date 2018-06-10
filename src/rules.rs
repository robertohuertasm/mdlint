use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

#[macro_export]
macro_rules! rule {
    ($name:ident : $desc:expr; $func:expr) => {{
        pub struct $name {}

        impl RuleCheck for $name {
            fn check(&self, root: &AstNode) -> Option<RuleResult> {
                println!("Checking rule2 {}", $desc);
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

pub fn get4() -> impl RuleCheck {
    rule!{ Rule4: "from macro 4"; |desc: &str, root: &AstNode| {
        println!("{}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }}
}

pub fn get5() -> impl RuleCheck {
    fn f(desc: &str, root: &AstNode) -> Option<RuleResult> {
        println!("{}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }
    rule!{ Rule5: "from macro 5"; f}
}

pub fn get6() -> Box<impl RuleCheck> {
    fn f(desc: &str, root: &AstNode) -> Option<RuleResult> {
        println!("{}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }
    boxedrule!{ Rule6: "from macro 6"; f}
}
