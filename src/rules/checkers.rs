use comrak::nodes::AstNode;
use parser::{filter_nodes, is_heading};
use ruleset::RuleResult;

pub fn check_md001<'a>(root: &'a AstNode<'a>) -> RuleResult {
    println!("evaluating MD001...");
    let headings = filter_nodes(root.children(), is_heading);
    println!("{}", headings.len());
    RuleResult::new(
        "Header levels should only increment by one level at a time",
        "WTF!",
    )
}

// #[cfg(test)]
// mod test {

//     use super::*;
//     use parser::get_ast;
//     use typed_arena::Arena;

//     #[test]
//     fn it_has_details_if_ko() {
//         let arena = Arena::new();
//         let root = get_ast("fixtures/md001/md001.ko.md", &arena);
//         let result = check_md001(root);
//         assert!(result.details.is_some());
//     }

//     #[test]
//     fn it_does_not_have_details_if_all_ok() {
//         let arena = Arena::new();
//         let root = get_ast("fixtures/md001/md001.ok.md", &arena);
//         let result = check_md001(root);
//         assert!(result.details.is_none());
//     }
// }
