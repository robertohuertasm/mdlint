use parser;
use comrak::nodes::AstNode;
use typed_arena::Arena;


pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Box<RuleCheck>>,
}

pub struct RuleResult {
    pub description: String,
    pub info: String,
}

impl RuleResult {
    pub fn new(description: &str, info: &str) -> Self {
        RuleResult {
            description: description.to_string(),
            info: info.to_string(),
        }
    }
}

pub trait RuleCheck {
    fn check(&self, &AstNode) -> Option<RuleResult>;
}

impl RuleSet {
    pub fn run(&self, file_path: &str) -> Vec<RuleResult> {
        let arena = Arena::new();
        let root = parser::get_ast(file_path, &arena);

        self.rules
            .iter()
            .map(|r| r.check(root))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect()
    }
}