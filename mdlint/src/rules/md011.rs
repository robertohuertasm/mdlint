use comrak::nodes::AstNode;
use crate::parser::{extract_content, flatten_nodes_with_content};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use regex::Regex;

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let nodes = flatten_nodes_with_content(root);

    let mut details: Vec<RuleResultDetails> = Vec::new();
    let rx = Regex::new(r"\([^)]+\)\[[^\]]+\]").unwrap();
    nodes.iter().for_each(|x| {
        let content = extract_content(x);
        
        // println!("{:?}", x.data.borrow());
        // println!("\nINNER \n");
        // x.children().for_each(|c| println!("{:?}", c.data.borrow()));
        let mut last_parsed_line = 0;
        content.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
            // condition
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
                // custom print
                details.push(RuleResultDetails::new(
                    last_parsed_line,
                    node.start_column,
                    l.to_string(),
                ));
            }
        });
    });

    RuleResult::new(
        "MD011",
        "no-reversed-links",
        "Reversed link syntax",
        details.to_option(),
    )
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use crate::rules::common_tests;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        common_tests::all_ok("fixtures/md011/md011_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md011/md011_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 3);
        let first = &details[0];
        assert_eq!(first.line, 3);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "(Proper link)[https://www.rust-lang.org]");
        let second = &details[1];
        assert_eq!(second.line, 5);
        assert_eq!(second.column, 3);
        assert_eq!(second.content, "(Proper link)[https://www.rust-lang.org]");
    }
}
