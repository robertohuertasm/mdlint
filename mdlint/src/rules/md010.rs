use comrak::nodes::AstNode;
use crate::rules::common_checks::check_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::RuleResult;

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let details = check_content(root, r"\t", None);

    RuleResult::new("MD010", "no-hard-tabs", "Hard tabs", details.to_option())
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parser::get_ast;
    use crate::rules::common_tests;
    use typed_arena::Arena;

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        common_tests::all_ok("fixtures/md010/md010_ok.md", Box::new(check));
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
