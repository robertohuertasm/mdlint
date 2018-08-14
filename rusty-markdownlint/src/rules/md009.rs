use comrak::nodes::AstNode;
use crate::parser::extract_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    root.children().for_each(|x| {
        let content = extract_content(x);
        if content.ends_with(' ') || content.ends_with(" \n") {
            let node = x.data.borrow();
            details.push(RuleResultDetails::from_node(&node));
        }
    });

    RuleResult::new(
        "MD009",
        "no-trailing-spaces",
        "Trailing spaces",
        details.to_option(),
    )
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md009/md009_ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md009/md009_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 2);
        let first = &details[0];
        assert_eq!(first.line, 3);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "This is my ok document \n");
    }
}
