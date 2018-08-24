#![cfg(test)]
use crate::parser::get_ast;
use crate::ruleset::CheckFn;
use typed_arena::Arena;

crate fn all_ok(file: &str, check: CheckFn) {
    let arena = Arena::new();
    let a = &arena;
    let root = get_ast(file, a);
    let result = check(root);
    assert!(result.details.is_none());
}
