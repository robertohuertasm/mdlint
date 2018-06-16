use comrak::nodes::AstNode;
use console::style;
use emoji;
use parser;
use std::fmt;
use typed_arena::Arena;

pub trait RuleCheck {
    fn check<'a>(&self, node: &'a AstNode<'a>) -> RuleResult;
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
    pub name: String,
    pub description: String,
    pub details: Option<Vec<RuleResultDetails>>,
}

impl RuleResult {
    pub fn new(name: &str, description: &str, details: Option<Vec<RuleResultDetails>>) -> Self {
        RuleResult {
            name: name.to_string(),
            description: description.to_string(),
            details,
        }
    }

    pub fn to_string(&self) -> String {
        let mut final_str = format!(
            "{}{}\r\n{}\r\n\r\n",
            emoji::ERROR,
            style(&self.name).bold().red(),
            style(&self.description).underlined().yellow()
        );
        if let Some(ref details) = self.details {
            details.iter().for_each(|detail| {
                final_str.push_str(&format!("{}   {}", emoji::INFO, detail.to_string()));
            });
        }
        final_str
    }
}

impl fmt::Display for RuleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
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

    pub fn to_string(&self) -> String {
        format!(
            "ln. {}, col. {}: '{}'",
            self.line, self.column, self.content
        )
    }
}

impl fmt::Display for RuleResultDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
