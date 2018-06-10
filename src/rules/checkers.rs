use comrak::nodes::AstNode;
use ruleset::RuleResult;

pub fn check_md001(desc: &str, root: &AstNode) -> Option<RuleResult> {
    println!("evaluating: {}", desc);
    Some(RuleResult::new(desc, "WTF!"))
}

