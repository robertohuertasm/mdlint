use comrak::nodes::AstNode;
use crate::parser::is_codeblock;
use crate::rules::common_checks::check_content;
use crate::rules::extensions::VecExt;
use crate::ruleset::{RuleResult, RuleResultDetails};
use regex::Regex;

crate fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let rx = Regex::new(r"^$|\$\s.*$").unwrap();
    let details: Vec<RuleResultDetails> = check_content(root, r"^\$\s", Some(is_codeblock))
        .into_iter()
        .filter(|d| match d.next_content {
            Some(ref line) if rx.is_match(line) => true,
            _ => false,
        }).collect();

    RuleResult::new(
        "MD014",
        "commands-show-output",
        "Dollar signs used before commands without showing output",
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
        common_tests::all_ok("fixtures/md014/md014_ok.md", Box::new(check));
    }

    #[test]
    fn it_has_details_if_ko() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md014/md014_ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 3);
        let first = &details[0];
        assert_eq!(first.line, 12);
        assert_eq!(first.column, 5);
        assert_eq!(first.content, "$ this should be invalid");
        let second = &details[1];
        assert_eq!(second.line, 14);
        assert_eq!(second.column, 5);
        assert_eq!(second.content, "$ this should be also invalid");
        let third = &details[2];
        assert_eq!(third.line, 15);
        assert_eq!(third.column, 5);
        assert_eq!(third.content, "$ this should be invalid too");
    }
}
