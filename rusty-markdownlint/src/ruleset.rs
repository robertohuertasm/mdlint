use comrak::nodes::{Ast, AstNode};
use console::style;
use crate::emoji;
use crate::parser;
use std::{cell::Ref, fmt};
use typed_arena::Arena;

crate type CheckFn<'a> = dyn Fn(&'a AstNode<'a>) -> RuleResult;

crate struct RuleSet<'a> {
    crate arena: &'a Arena<AstNode<'a>>,
    crate rules: Vec<Box<CheckFn<'a>>>,
}

impl<'a> RuleSet<'a> {
    crate fn new(rules: Vec<Box<CheckFn<'a>>>, arena: &'a Arena<AstNode<'a>>) -> RuleSet<'a> {
        RuleSet { rules, arena }
    }

    crate fn run(&self, file_path: &str) -> Vec<RuleResult> {
        let root = parser::get_ast(file_path, &self.arena);
        self.rules
            .iter()
            .map(|f| f(root))
            .filter(|r| r.details.is_some())
            .collect()
    }
}

#[derive(Debug)]
pub struct RuleResult {
    pub name: String,
    pub alias: String,
    pub description: String,
    pub details: Option<Vec<RuleResultDetails>>,
}

impl RuleResult {
    crate fn new(
        name: &str,
        alias: &str,
        description: &str,
        details: Option<Vec<RuleResultDetails>>,
    ) -> Self {
        RuleResult {
            name: name.to_string(),
            alias: alias.to_string(),
            description: description.to_string(),
            details,
        }
    }

    crate fn to_string(&self) -> String {
        let title = format!("{}/{}", self.name, self.alias);
        let mut final_str = format!(
            "{}{}\r\n{}\r\n",
            emoji::ERROR,
            style(title).bold().red(),
            style(&self.description).underlined().yellow()
        );
        if let Some(ref details) = self.details {
            details.iter().for_each(|detail| {
                final_str.push_str(&format!("\r\n{}{}", emoji::INFO, detail.to_string()));
            });
        }
        final_str
    }
}

impl fmt::Display for RuleResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    crate fn new(line: u32, column: usize, content: String) -> Self {
        RuleResultDetails {
            line,
            column,
            content,
        }
    }

    crate fn from_node(node: &Ref<'_, Ast>) -> Self {
        RuleResultDetails::new(
            node.start_line,
            node.start_column,
            parser::content_to_string(node.content.to_vec()),
        )
    }

    crate fn to_string(&self) -> String {
        format!("ln. {}, col. {}: {}", self.line, self.column, self.content)
    }
}

impl fmt::Display for RuleResultDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
