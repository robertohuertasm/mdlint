use comrak::nodes::{AstNode, Ast};
use console::style;
use crate::emoji;
use crate::parser;
use std::{fmt, cell::Ref};
use typed_arena::Arena;

crate trait RuleCheck {
    fn check<'a>(&self, node: &'a AstNode<'a>) -> RuleResult;
}

crate struct RuleSet {
    crate name: String,
    crate rules: Vec<Box<dyn RuleCheck>>,
}

impl RuleSet {
    crate fn run(&self, file_path: &str) -> Vec<RuleResult> {
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
crate struct RuleResult {
    crate name: String,
    crate alias: String,
    crate description: String,
    crate details: Option<Vec<RuleResultDetails>>,
}

impl RuleResult {
    crate fn new(name: &str, alias: &str, description: &str, details: Option<Vec<RuleResultDetails>>) -> Self {
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
crate struct RuleResultDetails {
    crate line: u32,
    crate column: usize,
    crate content: String,
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
            parser::content_to_string(node.content.to_vec())
        )
    }

    crate fn to_string(&self) -> String {
        format!(
            "ln. {}, col. {}: {}",
            self.line, self.column, self.content
        )
    }
}

impl fmt::Display for RuleResultDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
