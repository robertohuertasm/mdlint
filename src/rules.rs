use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

#[macro_export]
macro_rules! rule {
    ($name:ident : $value:expr) => {{
        pub struct $name {
            description: String,
        }

        impl $name {
            pub fn new(description: &str) -> $name {
                $name {
                    description: description.to_string(),
                }
            }
        }

        impl RuleCheck for $name {
            fn check(&self, _root: &AstNode) -> Option<RuleResult> {
                println!("Checking rule {}", self.description);
                Some(RuleResult {
                    description: self.description.clone(),
                    info: "Info loca".to_string(),
                })
            }
        }
        $name::new($value)
    }};
}

#[macro_export]
macro_rules! rule2 {
    ($name:ident : $desc:expr; $func:expr) => {{
        pub struct $name {}


        impl RuleCheck for $name {
            fn check(&self, root: &AstNode) -> Option<RuleResult> {
                println!("Checking rule2 {}", $desc);
                $func($desc, root)
            }
        }
        $name{}
    }};
}

pub fn get1() -> impl RuleCheck {
    rule!{Rule1: "from macro 1"}
}

pub fn get2() -> impl RuleCheck {
    rule!{Rule2: "from macro 2"}
}

pub fn get3() -> impl RuleCheck {
    Rule::new("from hand")
}

pub fn get4() -> impl RuleCheck {
    rule2!{ Rule4: "from macro 4"; |desc: &str, root: &AstNode| {
        println!("{}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }}
}

pub fn get5() -> impl RuleCheck {
    fn f(desc: &str, root: &AstNode) -> Option<RuleResult> {
        println!("{}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }
    rule2!{ Rule4: "from macro 5"; f}
}

pub struct Rule {
    pub description: String,
}

impl Rule {
    pub fn new(description: &str) -> Rule {
        Rule {
            description: description.to_string(),
        }
    }
}

impl RuleCheck for Rule {
    fn check(&self, _root: &AstNode) -> Option<RuleResult> {
        println!("Checking rule {}", self.description);
        Some(RuleResult {
            description: self.description.clone(),
            info: "Info local".to_string(),
        })
    }
}
