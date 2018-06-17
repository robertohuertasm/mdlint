use comrak::nodes::{AstNode, Ast, NodeValue};
use parser::{filter_nodes, is_heading};
use ruleset::{RuleResult, RuleResultDetails};
use rules::extensions::VecExt;
use std::cell::Ref;

pub fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    if let Some(heading) = filter_nodes(root.children(), is_heading).first() {
        let node: Ref<Ast> = heading.data.borrow();
        if let NodeValue::Heading(x) = node.value {
            if x.level !=1 {
                details.push(RuleResultDetails::from_node(&node));
            }
        }
    }

    RuleResult::new(
        "MD002",
        "first-header-h1",
        "First header should be a top level header",
        details.to_option()
    )
}

#[cfg(test)]
mod test {

    use super::*;
    use parser::get_ast;
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
        let root = get_ast("fixtures/md002/md002_no_headings.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
