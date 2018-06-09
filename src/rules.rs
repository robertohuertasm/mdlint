use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

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
                println!("Checking rule2 {}", self.description);
                Some(RuleResult {
                    description: self.description.clone(),
                    info: "Info loca".to_string(),
                })
            }
        }
        $name::new($value)
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
        println!("{:?}", vec2![1, 2, 3, 4]);
        Some(RuleResult {
            description: self.description.clone(),
            info: "Info loca".to_string(),
        })
    }
}
