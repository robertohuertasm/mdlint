use comrak::nodes::AstNode;
use ruleset::{RuleCheck, RuleResult};

pub fn rule() -> Box<impl RuleCheck> {
    fn f(desc: &str, root: &AstNode) -> Option<RuleResult> {
        println!("evaluating: {}", desc);
        Some(RuleResult::new(desc, "WTF!"))
    }
    boxedrule!{ Rule7: "from macro 7"; f}
}
