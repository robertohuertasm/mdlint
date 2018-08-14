use comrak::nodes::{AstNode, NodeValue};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    if let Some(first_line) = root.children().nth(0) {
        let node = first_line.data.borrow();

        match node.value {
            NodeValue::Heading(x) if x.level == 1 => (),
            _ => details.push(RuleResultDetails::from_node(&node)),
        };
    }

    RuleResult::new(
        "MD041",
        "first-line-h1",
        "First line in file should be a top level header",
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
        let root = get_ast("fixtures/md041/md041_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 1);
        assert_eq!(first.column, 1);
        assert!(first.content.starts_with("first line"));
    }

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md041/md041_ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn if_no_headers_it_fails() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md041/md041_no_items.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 1);
        assert_eq!(first.column, 1);
        assert!(first.content.starts_with("A version of this"));
    }
}
