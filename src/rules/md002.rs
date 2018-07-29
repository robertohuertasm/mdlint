use comrak::nodes::{Ast, AstNode, NodeValue};
use crate::parser::{filter_nodes, is_heading};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use std::cell::Ref;

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    if let Some(heading) = filter_nodes(root.children(), is_heading).first() {
        let node: Ref<'_, Ast> = heading.data.borrow();
        if let NodeValue::Heading(x) = node.value {
            if x.level != 1 {
                details.push(RuleResultDetails::from_node(&node));
            }
        }
    }

    RuleResult::new(
        "MD002",
        "first-header-h1",
        "First header should be a top level header",
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
        let root = get_ast("fixtures/md002/md002_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 1);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "Test");
    }

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md002/md002_ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_no_headers() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md002/md002_no_items.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
