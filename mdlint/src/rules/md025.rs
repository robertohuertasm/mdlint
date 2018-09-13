use comrak::nodes::AstNode;
use crate::parser::{filter_nodes, is_heading_1};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let details: Vec<RuleResultDetails> = filter_nodes(root, is_heading_1)
        .into_iter()
        .skip(1)
        .map(|x| x.data.borrow())
        .fold(Vec::new(), |mut acc, node| {
            acc.push(RuleResultDetails::from_node(&node));
            acc
        });

    RuleResult::new(
        "MD025",
        "single-h1",
        "Multiple top level headers in the same document",
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
        common_tests::all_ok("fixtures/md025/md025_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md025/md025_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 2);
        let first = &details[0];
        assert_eq!(first.line, 9);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "TITLE3");
        let second = &details[1];
        assert_eq!(second.line, 13);
        assert_eq!(second.column, 1);
        assert_eq!(second.content, "TITLE4");
    }
}
