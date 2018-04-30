#![allow(dead_code)]
extern crate comrak;
extern crate typed_arena;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, ComrakOptions};
use typed_arena::Arena;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let file = "data/test1.md";
    let text = read_file(file).expect(&format!("Failed to find file: {}", file));
    // println!("{}", text);
    let arena = Arena::new();
    let root = parse_document(&arena, &text, &ComrakOptions::default());

    let headings = f(root.children());
    headings.into_iter()
    .for_each(|x| println!("{:?}", x.data.borrow_mut()));

    // let vv = root.children().nth(1).unwrap();
    // check_second_child(&vv);
    // let vv2 = root.children().nth(0).unwrap();
    // check_second_child(&vv2);

    // let v: Vec<&AstNode> = root.children().collect();
    // check_first_child(&root);
    // check_second_child(&v[0]);
    // let headers = find_headings(&v);
    // headers
    //     .into_iter()
    //     .for_each(|x| println!("{:?}", x.data.borrow_mut()));
}

// TODO: pass the is_heading function as a fn param
fn f<'a, T>(i: T) -> Vec<&'a AstNode<'a>>
where
    T: Iterator<Item = &'a AstNode<'a>>,
{
    i.filter(|x| is_heading(&x.data.borrow_mut().value))
        .collect::<Vec<&AstNode>>()
}

fn find_headings<'a>(nodes: &[&'a AstNode<'a>]) -> Vec<&'a AstNode<'a>> {
    nodes
        .iter()
        .cloned()
        .filter(|x| is_heading(&x.data.borrow_mut().value))
        .collect::<Vec<&AstNode>>()
}

fn is_heading(node: &NodeValue) -> bool {
    match node {
        &NodeValue::Heading(_) => true,
        _ => false,
    }
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut tokens = String::new();
    let mut f = File::open(Path::new(file_path))?;
    f.read_to_string(&mut tokens)?;
    Ok(tokens)
}

fn check_first_child(node: &AstNode) {
    let fc = node.first_child().ok_or("mierda").unwrap();
    let st = fc.data.borrow_mut().content.to_vec();
    println!("{:?}", String::from_utf8(st).unwrap());
    println!("{:?}", fc.data.borrow_mut());
}

fn check_second_child(node: &AstNode) {
    println!("{}", extract_content(&node));
    println!("{:?}", node.data.borrow_mut());
}

fn extract_content(node: &AstNode) -> String {
    let st = node.data.borrow_mut().content.to_vec();
    String::from_utf8(st).expect("Something went wrong while transforming content to string")
}

fn use_cmark() {
    let arena = Arena::new();

    let md = Path::new("data/test1.md");
    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();

    let root = parse_document(&arena, &tokens, &ComrakOptions::default());
    let fc = root.first_child().ok_or("mierda").unwrap();
    let st = fc.data.borrow_mut().content.to_vec();
    println!("{:?}", String::from_utf8(st).unwrap());
    println!("{:?}", fc.data.borrow_mut());

    let v: Vec<_> = root.children().collect();
    println!("{:?}", v[1].data.borrow_mut());

    // for node in root.children() {
    //     println!("{:?}", node);
    // }

    // let mut md = vec![];
    // format_commonmark(root, &ComrakOptions::default(), &mut md).unwrap();
    // let result2 = String::from_utf8(md).unwrap();
    // println!("{}", result2);
}

fn use_cmark2() {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
        &ComrakOptions::default(),
    );

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::Text(ref mut text) => {
            let orig = std::mem::replace(text, vec![]);
            *text = String::from_utf8(orig)
                .unwrap()
                .replace("my", "your")
                .as_bytes()
                .to_vec();
        }
        _ => (),
    });

    // let mut html = vec![];
    // format_html(root, &ComrakOptions::default(), &mut html).unwrap();
    // let result = String::from_utf8(html).unwrap();
    // assert_eq!(
    //     result,
    //     "<p>This is your input.</p>\n\
    //      <ol>\n\
    //      <li>Also your input.</li>\n\
    //      <li>Certainly your input.</li>\n\
    //      </ol>\n"
    // );
    // println!("{}", result);
}
