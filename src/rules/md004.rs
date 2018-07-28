use comrak::nodes::{Ast, AstNode, NodeValue};
use crate::parser::{filter_nodes, is_ul};
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use std::cell::Ref;

pub fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    let mut li_type: u8 = 42; // dash
    let uls = filter_nodes(root.children(), is_ul);

    uls.into_iter()
        .map(|x| x.data.borrow())
        .enumerate()
        .for_each(|(i, node): (usize, Ref<Ast>)| {
            if let NodeValue::List(x) = node.value {
                if i == 0 {
                    li_type = x.bullet_char;
                } else {
                    if x.bullet_char != li_type {
                        details.push(RuleResultDetails::new(
                            node.start_line,
                            node.start_column,
                            format!("[Expected: {}; Actual: {}]", li_type as char, x.bullet_char as char),
                        ));
                    }
                }
            }
        });

    RuleResult::new(
        "MD004",
        "ul-style",
        "Unordered list style",
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
        let root = get_ast("fixtures/md004/md004_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 3);
        let first = &details[0];
        assert_eq!(first.line, 7);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "[Expected: *; Actual: -]");
    }

    #[test]
    fn it_does_not_have_details_if_all_ok_asterisk() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md004/md004_ok_asterisk.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_all_ok_dash() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md004/md004_ok_dash.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_all_ok_plus() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md004/md004_ok_plus.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_no_items() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md004/md004_no_items.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_does_not_have_details_if_no_unordered() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md004/md004_no_unordered.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
