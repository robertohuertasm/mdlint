use comrak::nodes::AstNode;
use crate::parser::extract_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let mut details: Vec<RuleResultDetails> = Vec::new();
    root.children().for_each(|x| {
        let content = extract_content(x);
        let mut last_parsed_line = 0;
        content.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
            if l.contains('\t') {
                let node = x.data.borrow();
                last_parsed_line = if last_parsed_line == node.start_line {
                    last_parsed_line += 1;
                    last_parsed_line
                } else {
                    node.start_line
                };
                details.push(RuleResultDetails::new(
                    last_parsed_line,
                    node.start_column,
                    l.to_string(),
                ));
            }
        });
    });

    RuleResult::new("MD010", "no-hard-tabs", "Hard tabs", details.to_option())
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md010/md010_ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md010/md010_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 3);
        assert_eq!(first.column, 2);
        assert!(first.content.starts_with("\tThis is my ok document"));
    }

    #[test]
    fn it_has_details_if_ko_with_single_line() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md010/md010_ko_single_line.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 2);
        let first = &details[0];
        assert_eq!(first.line, 3);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "This is my ok document\t ");
        let second = &details[1];
        assert_eq!(second.line, 6);
        assert_eq!(second.column, 1);
        assert_eq!(second.content, "This is my ok document\t2Another ok line");
    }
}
