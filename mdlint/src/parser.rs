use comrak::nodes::{Ast, AstNode, ListType, NodeValue};
use comrak::{parse_document, ComrakOptions};
use typed_arena::Arena;

use std::cell::Ref;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

crate fn get_ast<'a>(path: &str, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
    let text = read_file(path).unwrap_or_else(|_| panic!("Failed to find file: {}", path));
    parse_document(arena, &text, &ComrakOptions::default())
}

crate fn read_file(file_path: &str) -> Result<String, Error> {
    let mut tokens = String::new();
    let mut f = File::open(Path::new(file_path))?;
    f.read_to_string(&mut tokens)?;
    Ok(tokens)
}

crate fn extract_content(node: &AstNode<'_>) -> String {
    extract_content_from_node(&node.data.borrow())
}

crate fn extract_content_from_node(node: &Ref<'_, Ast>) -> String {
    if let NodeValue::CodeBlock(x) = node.value.clone() {
        let st = content_to_string(x.literal.to_vec());
        if node.start_column < 4 {
            format!("\t{}", st)
        } else {
            st
        }
    } else {
        content_to_string(node.content.to_vec())
    }
}

crate fn content_to_string(content: Vec<u8>) -> String {
    String::from_utf8(content).expect("Something went wrong while transforming content to string")
}

#[allow(dead_code)]
crate fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

crate fn flatten_nodes<'a>(node: &'a AstNode<'a>) -> Vec<&'a AstNode<'a>> {
    traverse_nodes(node, None)
}

crate fn flatten_nodes_with_content<'a>(node: &'a AstNode<'a>) -> Vec<&'a AstNode<'a>> {
    flatten_nodes(node)
        .into_iter()
        .filter(|child| {
            let n = child.data.borrow();
            let has_content = !n.content.is_empty();
            if has_content {
                true
            } else {
                if let NodeValue::CodeBlock(ref x) = &n.value {
                    return !x.literal.is_empty();
                }
                false
            }
        }).collect()
}

crate fn filter_nodes<'a>(
    node: &'a AstNode<'a>,
    filter_fn: fn(&NodeValue) -> bool,
) -> Vec<&'a AstNode<'a>> {
    traverse_nodes(node, Some(filter_fn))
}

fn traverse_nodes<'a>(
    node: &'a AstNode<'a>,
    filter_fn: Option<fn(&NodeValue) -> bool>,
) -> Vec<&'a AstNode<'a>> {
    let mut final_vec = Vec::new();
    // println!("Node");
    for child in node.children() {
        // println!("------");
        // println!("{:?}", child.data.borrow());
        // println!("{}", extract_content(child));
        if let Some(ffn) = filter_fn {
            if ffn(&child.data.borrow_mut().value) {
                final_vec.push(child);
            }
        } else {
            final_vec.push(child);
        }

        let inner_vec = traverse_nodes(child, filter_fn);
        final_vec.extend(inner_vec);
    }
    final_vec
}

crate fn is_heading(node: &NodeValue) -> bool {
    match node {
        NodeValue::Heading(_) => true,
        _ => false,
    }
}

crate fn is_ul(node: &NodeValue) -> bool {
    match node {
        NodeValue::List(x) if x.list_type == ListType::Bullet => true,
        _ => false,
    }
}

#[allow(dead_code)]
crate fn is_ol(node: &NodeValue) -> bool {
    match node {
        NodeValue::List(x) if x.list_type == ListType::Ordered => true,
        _ => false,
    }
}

#[allow(dead_code)]
crate fn is_paragraph(node: &NodeValue) -> bool {
    match node {
        NodeValue::Paragraph => true,
        _ => false,
    }
}
