use comrak::nodes::{Ast, AstNode, NodeValue};
use crate::parser::{extract_content, filter_nodes, flatten_nodes_with_content};
use crate::ruleset::RuleResultDetails;
use regex::Regex;

crate fn check_content<'a>(
    root: &'a AstNode<'a>,
    regex: &str,
    filter_fn: Option<fn(&NodeValue) -> bool>,
) -> Vec<RuleResultDetails> {
    let nodes = match filter_fn {
        Some(ffn) => filter_nodes(root, ffn),
        None => flatten_nodes_with_content(root),
    };
    let rx = Regex::new(regex).unwrap();
    nodes.iter().fold(Vec::new(), |mut acc, &n| {
        let mut items = check_content_for_node(n, &rx);
        if !items.is_empty() {
            acc.append(&mut items);
        }
        acc
    })
}

fn check_content_for_node<'a>(node: &'a AstNode<'a>, rx: &Regex) -> Vec<RuleResultDetails> {
    let mut results: Vec<RuleResultDetails> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let Ast {
        start_line,
        start_column,
        ..
    } = node.data.borrow().clone();
    let mut line_number = start_line;
    extract_content(node).split('\n').for_each(|line| {
        // set this line as the next content of the previous resultSet
        if !results.is_empty() && results.last().unwrap().line == line_number - 1 {
            let mut last_resultset = results.pop().unwrap();
            last_resultset.next_content = Some(line.to_string());
            results.push(last_resultset);
        }

        if rx.is_match(line) {
            let mut rs = RuleResultDetails::new(line_number, start_column, line.to_string());
            // set previous line for this resultSet
            if !lines.is_empty() {
                rs.previous_content = lines.last().cloned();
            }
            results.push(rs);
        }
        line_number += 1;
        lines.push(line.to_string());
    });
    results
}
