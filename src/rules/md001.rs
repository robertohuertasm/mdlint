use comrak::nodes::{Ast, AstNode, NodeValue};
use crate::parser::{filter_nodes, is_heading};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use std::cell::Ref;

pub fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut prev_level = 0;
    let mut details: Vec<RuleResultDetails> = Vec::new();
    let headings = filter_nodes(root.children(), is_heading);

    headings
        .into_iter()
        .map(|x| x.data.borrow())
        .for_each(|node: Ref<Ast>| {
            if let NodeValue::Heading(x) = node.value {
                let current_level = x.level;
                if current_level > prev_level + 1 {
                    details.push(RuleResultDetails::from_node(&node));
                }
                prev_level = current_level;
            }
        });

    RuleResult::new(
        "MD001",
        "header-increment",
        "Header levels should only increment by one level at a time",
        details.to_option(),
    )
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use typed_arena::Arena;

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md001/md001_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 9);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "TITLE3");
    }

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md001/md001_ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_no_headers() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md001/md001_no_items.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
