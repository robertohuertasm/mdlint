use comrak::nodes::AstNode;
use ruleset::RuleResult;

pub fn check_md001(desc: &str, root: &AstNode) -> RuleResult {
    println!("evaluating: {}", desc);
    RuleResult::new(desc, "WTF!")
}

