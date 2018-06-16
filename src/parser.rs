use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, ComrakOptions};
use typed_arena::Arena;

use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub fn get_ast<'a>(path: &str, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
    let text = read_file(path).expect(&format!("Failed to find file: {}", path));
    let root = parse_document(arena, &text, &ComrakOptions::default());
    root
}

pub fn read_file(file_path: &str) -> Result<String, Error> {
    let mut tokens = String::new();
    let mut f = File::open(Path::new(file_path))?;
    f.read_to_string(&mut tokens)?;
    Ok(tokens)
}

// pub fn extract_content(node: &AstNode) -> String {
//     let data = node.data.borrow().content.to_vec();
//     content_to_string(data)
// }

pub fn content_to_string(content: Vec<u8>) -> String {
    String::from_utf8(content).expect("Something went wrong while transforming content to string")
}

pub fn filter_nodes<'a, T>(node: T, filter_fn: fn(&NodeValue) -> bool) -> Vec<&'a AstNode<'a>>
where
    T: Iterator<Item = &'a AstNode<'a>>,
{
    node.filter(|x| filter_fn(&x.data.borrow_mut().value))
        .collect::<Vec<&AstNode>>()
}

pub fn is_heading(node: &NodeValue) -> bool {
    match node {
        &NodeValue::Heading(_) => true,
        _ => false,
    }
}
