use comrak::nodes::AstNode;
use parser;
use typed_arena::Arena;

pub trait RuleCheck {
    fn check<'a>(&self, &'a AstNode<'a>) -> RuleResult;
}

pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Box<RuleCheck>>,
}

impl RuleSet {
    pub fn run(&self, file_path: &str) -> Vec<RuleResult> {
        let arena = Arena::new();
        let root = parser::get_ast(file_path, &arena);

        // TODO: Use this to know the format of the returning nodes
        /*
        let headings = parser::filter_nodes(root.children(), parser::is_heading);
        headings
            .into_iter()
            .for_each(|x| println!("{:?}", x.data.borrow_mut()));
        */

        self.rules
            .iter()
            .map(|r| r.check(root))
            .filter(|r| r.details.is_some())
            .collect()
    }
}

#[derive(Debug)]
pub struct RuleResult {
    pub description: String,
    pub details: Option<Vec<RuleResultDetails>>,
}

impl RuleResult {
    pub fn new(description: &str, details: Option<Vec<RuleResultDetails>>) -> Self {
        RuleResult {
            description: description.to_string(),
            details,
        }
    }
}

#[derive(Debug)]
pub struct RuleResultDetails {
    pub line: u32,
    pub column: usize,
    pub content: String,
}

impl RuleResultDetails {
    pub fn new(line: u32, column: usize, content: String) -> Self {
        RuleResultDetails {
            line,
            column,
            content,
        }
    }
}
