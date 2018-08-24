use comrak::nodes::AstNode;
use crate::parser::{extract_content, flatten_nodes_with_content};
use crate::ruleset::RuleResultDetails;
use regex::Regex;

crate fn check_content<'a>(root: &'a AstNode<'a>, regex: &str) -> Vec<RuleResultDetails> {
    let nodes = flatten_nodes_with_content(root);

    let mut details: Vec<RuleResultDetails> = Vec::new();
    let rx = Regex::new(regex).unwrap();
    nodes.iter().for_each(|x| {
        let content = extract_content(x);
        // println!("{:?}", x.data.borrow());
        // println!("\nINNER \n");
        // x.children().for_each(|c| println!("{:?}", c.data.borrow()));
        let mut last_parsed_line = 0;
        content.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
            if rx.is_match(l) {
                // println!("\n------- \n");
                // println!("{}", content);
                let node = x.data.borrow();
                last_parsed_line = if last_parsed_line == node.start_line {
                    last_parsed_line += 1;
                    last_parsed_line
                } else {
                    node.start_line
                };
                details.push(RuleResultDetails::new(
                    last_parsed_line,
                    node.start_column,
                    l.to_string(),
                ));
            }
        });
    });
    details
}
