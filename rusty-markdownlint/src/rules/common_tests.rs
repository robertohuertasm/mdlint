#![cfg(test)]
use crate::parser::get_ast;
// use crate::ruleset::CheckFn;
use comrak::nodes::AstNode;
use crate::ruleset::RuleResult;
use typed_arena::Arena;

type CheckFn = dyn for<'a> Fn(&'a AstNode<'a>) -> RuleResult;

crate fn all_ok(file: &str, check: Box<CheckFn>) {
    let arena = Arena::new();
    let a = &arena;
    let root = get_ast(file, a);
    let result = check(root);
    assert!(result.details.is_none());
}
