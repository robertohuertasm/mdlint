use comrak::nodes::AstNode;
use crate::parser::flatten_nodes_with_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    // multimple blanklines in code blocks should be valid
    let mut details: Vec<RuleResultDetails> = Vec::new();
    let nodes = flatten_nodes_with_content(root);
    nodes.iter().enumerate().for_each(|(i, n)| {
        if i == 0 {
            return;
        }
        let curr = n.data.borrow();
        let prev_end_line = nodes[i - 1].data.borrow().end_line;
        let curr_start_line = curr.start_line;
        if curr_start_line - prev_end_line > 2 {
            details.push(RuleResultDetails::from_node(&curr));
        }
    });

    RuleResult::new(
        "MD012",
        "no-multiple-blanks",
        "Multiple consecutive blank lines",
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
        common_tests::all_ok("fixtures/md012/md012_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md012/md012_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 3);
        let first = &details[0];
        assert_eq!(first.line, 4);
        assert_eq!(first.column, 1);
        let second = &details[1];
        assert_eq!(second.line, 9);
        assert_eq!(second.column, 1);
        assert_eq!(second.content, "This is another line\n");
        let second = &details[2];
        assert_eq!(second.line, 21);
        assert_eq!(second.column, 3);
        assert_eq!(second.content, "item two\n");
    }
}
