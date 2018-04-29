#![allow(dead_code)]
extern crate markdown;

extern crate prettify_cmark;
use prettify_cmark::prettify;

extern crate comrak;
extern crate typed_arena;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, format_html, parse_document, ComrakOptions};
use typed_arena::Arena;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // use_markdown();
    use_cmark();
    // use_prettify();
}

fn use_prettify() {
    let md = Path::new("data/test1.md");
    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();
    let output = prettify(&tokens);
    println!("{}", output);
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

    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();
    let result = String::from_utf8(html).unwrap();
    assert_eq!(
        result,
        "<p>This is your input.</p>\n\
         <ol>\n\
         <li>Also your input.</li>\n\
         <li>Certainly your input.</li>\n\
         </ol>\n"
    );
    println!("{}", result);
}

fn use_markdown() {
    let md = Path::new("data/test1.md");
    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();
    let md_tokens = markdown::tokenize(&tokens);
    md_tokens.iter().for_each(|x| println!("{:?}", x));

    let md_html = markdown::to_html(&tokens);
    println!("{}", md_html);
}
