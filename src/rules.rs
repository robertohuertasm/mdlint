use comrak::nodes::{AstNode};

type CheckFn = fn (&AstNode) -> RuleInfo;

pub struct RuleInfo {
    pub description: String,
    pub info: String,
}

pub struct RuleSet {
    pub rule_info: Vec<RuleInfo>,
}

pub trait Check {
    fn check(&self, &AstNode) -> Option<RuleInfo>;
}

pub struct Rule {
    pub description: String,
}

impl Rule {
    fn new(description: String) -> Rule {
        Rule { description }
    }
}

impl Check for Rule {
    fn check(&self, root: &AstNode) -> Option<RuleInfo> {
        println!("Checking rule {}", self.description);
        None
    }
}
