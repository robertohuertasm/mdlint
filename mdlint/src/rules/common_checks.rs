use comrak::nodes::AstNode;
use crate::parser::{extract_content, flatten_nodes_with_content};
use crate::ruleset::RuleResultDetails;
use regex::Regex;

crate fn check_content<'a>(root: &'a AstNode<'a>, regex: &str) -> Vec<RuleResultDetails> {
    let nodes = flatten_nodes_with_content(root);
    let rx = Regex::new(regex).unwrap();
    nodes.iter().fold(Vec::new(), |mut acc, &n| {
        let mut last_parsed_line = 0;
        extract_content(n)
            .split('\n')
            .filter(|line| !line.is_empty())
            .for_each(|line| {
                if rx.is_match(line) {
                    let node = n.data.borrow();
                    last_parsed_line = if last_parsed_line == node.start_line {
                        last_parsed_line += 1;
                        last_parsed_line
                    } else {
                        node.start_line
                    };
                    acc.push(RuleResultDetails::new(
                        last_parsed_line,
                        node.start_column,
                        line.to_string(),
                    ));
                }
            });
        acc
    })
}
