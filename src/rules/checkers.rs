use comrak::nodes::{AstNode, NodeValue};
use parser::{filter_nodes, is_heading};
use ruleset::{RuleResult, RuleResultDetails};

pub fn check_md001<'a>(root: &'a AstNode<'a>) -> RuleResult {
    println!("evaluating MD001...");
    let headings = filter_nodes(root.children(), is_heading);
    let prev: Option<&AstNode> = None;
    let details: Vec<RuleResultDetails> = Vec::new();
    let mut i: i8 = 0;
    headings.into_iter().for_each(|x| {
        i += 1;
        let xx = x.data.borrow();
        if let NodeValue::Heading(inner) = xx.value {
            println!("WTF ::> {:?}", inner.level);
        }
        println!("{} :: {:?}", i, xx.value);
    });
    RuleResult::new(
        "Header levels should only increment by one level at a time",
        "WTF!",
        None,
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
        let result = check_md001(root);
        assert!(result.details.is_some());
    }

    #[test]
    fn it_does_not_have_details_if_all_ok() {
        let arena = Arena::new();
        let root = get_ast("fixtures/md001/md001.ok.md", &arena);
        let result = check_md001(root);
        assert!(result.details.is_none());
    }
}
