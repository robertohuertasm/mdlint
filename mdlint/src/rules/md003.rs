use comrak::nodes::{Ast, AstNode, NodeValue};
use crate::parser::{filter_nodes, is_heading};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use std::cell::Ref;

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    let mut is_setext: bool = false;
    let headings = filter_nodes(root.children(), is_heading);

    headings
        .into_iter()
        .map(|x| x.data.borrow())
        .enumerate()
        .for_each(|(i, node): (usize, Ref<'_, Ast>)| {
            if let NodeValue::Heading(x) = node.value {
                if i == 0 {
                    is_setext = x.setext;
                } else if x.setext != is_setext {
                    details.push(RuleResultDetails::new(
                        node.start_line,
                        node.start_column,
                        format!(
                            "[Expected setext: {}; Actual setext: {}]",
                            is_setext, x.setext
                        ),
                    ));
                }
            }
        });

    RuleResult::new("MD003", "header-style", "Header style", details.to_option())
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use crate::rules::common_tests;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        common_tests::all_ok("fixtures/md003/md003_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md003/md003_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 5);
        assert_eq!(first.column, 1);
        assert_eq!(
            first.content,
            "[Expected setext: false; Actual setext: true]"
        );
    }

    #[test]
    fn it_does_not_have_details_if_no_headers() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md003/md003_no_items.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
