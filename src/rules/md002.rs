use comrak::nodes::{AstNode, NodeValue};
use parser::{content_to_string, filter_nodes, is_heading};
use ruleset::{RuleResult, RuleResultDetails};
use rules::VecExt;

pub fn check<'a>(root: &'a AstNode<'a>) -> RuleResult {
    let first_heading = filter_nodes(root.children(), is_heading);
    println!("{:?}", first_heading.first().data.borrow());

    let mut details: Vec<RuleResultDetails> = Vec::new();
    // headings.into_iter().map(|x| x.data.borrow()).for_each(|x| {
    //     if let NodeValue::Heading(node) = x.value {
    //         let current_level = node.level;
    //         if current_level > prev_level + 1 {
    //             details.push(RuleResultDetails::new(
    //                 x.start_line,
    //                 x.start_column,
    //                 content_to_string(x.content.to_vec()),
    //             ));
    //         }
    //         prev_level = current_level;
    //     }
    // });
    RuleResult::new(
        "MD002",
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
        let root = get_ast("fixtures/md001/md001.ko.md", &arena);
        let result = check(root);
        assert!(result.details.is_some());
        let details = result.details.unwrap();
        assert_eq!(details.len(), 1);
        let first = &details[0];
        assert_eq!(first.line, 9);
        assert_eq!(first.column, 1);
        assert_eq!(first.content, "TITLE3");
    }

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md001/md001.ok.md", &arena);
        let result = check(root);
        assert!(result.details.is_none());
    }
}
