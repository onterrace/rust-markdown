//! comrak 테스트

use regex::Regex;

#[warn(unused_variables)]


extern crate comrak;
extern crate jirepos_markdown;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};
// use comrak::{Node, parse_document, ComrakOptions, Tag};
// // use std::env;
// // use std::fs;
// // use std::fs::File;
// // use std::io::prelude::*;
// // use std::path::Path;
// // use jirepos_markdown::*;

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where  F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn make_markdown() {
    let arena = Arena::new();
    let root = parse_document(
        &arena,
        "This is my input.\n\n1. Also my input.\n  [your video](https://www.youtube.com/watch?v=9GKh3QJUj8Y&t=20) \n  Certainly my input.\n [my](http://my.ji.com)\n ![](.assets/test.png)",
        &ComrakOptions::default(),
    );

    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        // NodeValue가 text인 경우 
        &mut NodeValue::Text(ref mut text) => {
            // vector로 넘어온다 
            let val = String::from_utf8(text.to_vec()).unwrap();
            println!("val: {}", val);
            let orig = std::mem::replace(text, vec![]);
            *text = String::from_utf8(orig)
                .unwrap()
                .replace("my", "your")
                .as_bytes()
                .to_vec();
        },
        // NodeValue가 link인 경우
        &mut NodeValue::Link(ref mut link) => {
            let linkstr = String::from_utf8(link.url.to_vec()).unwrap();
            println!("linkstr: {}", linkstr);
            if is_youtube_url(&linkstr) {
                // youtube url 처리
                let embed_url = make_youtube_embed_url(extract_video_id(&linkstr));
                link.url = embed_url.as_bytes().to_vec();
            }
        },
        &mut NodeValue::Image(ref mut image) => {
            println!("=====image: {:?}", image);
            // .assets/test.png
            let imagestr = String::from_utf8(image.url.to_vec()).unwrap();
            println!("imagestr: {}", imagestr);
        },
        _ => (),
    });

    let mut html = vec![];
    
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();


    println!("{}", String::from_utf8(html).unwrap());

}





fn main() {
    
    make_markdown();
}



// fn main() {

//     let args: Vec<String> = env::args().collect();

//     // // println!("{:?}", args);
//     // // make code using env::args's count() method
//     // let arg_count = env::args().count();
//     // println!("count: {}", arg_count);

//     // if arg_count == 1 {
//     //     println!("No arguments");
//     // } else {
//     //     println!("{} arguments", arg_count);
//     // }

//     let s = jirepos_markdown::open_read(&args[1]);
//     println!("s: {}", s);
//     // let s = jirepos_markdown::to_markdown(&args[1]);
//     // println!("s: {}", s);

//     // let path_str: String = get_path(&args[1]);
//     // println!("path_str: {}", path_str);
//     // file_open(&args[1]);
//     // let result = file_type(&args[1]);
//     // match result {
//     //     Err(why) => panic!("couldn't open {}: {}", &args[1], why),
//     //     Ok(_) => println!("file_type: {:?}", result),
//     // };
//     // // Create a path to the desired file
//     // let path = Path::new(&args[1]);
//     // let display = path.display();
//     // println!("display: {}", display);

//     // // Open the path in read-only mode, returns `io::Result<File>`
//     // let mut file = match File::open(&path) {
//     //     Err(why) => panic!("couldn't open {}: {}", display, why),
//     //     Ok(file) => file,
//     // };

//     // // refer : https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
//     // // Read the file contents into a string, returns `io::Result<usize>`
//     // let mut s = String::new();
//     // match file.read_to_string(&mut s) {
//     //     Err(why) => panic!("couldn't read {}: {}", display, why),
//     //     Ok(_) => print!("{} contains:\n{}", display, s),
//     // }
// }



// fn main(){ 

//     let markdown = "# My heading\nSome text\n";
//     let arena = Arena::new();
//     let ast = parse_document(
//         &arena,
//         &markdown,
//         &ComrakOptions::default(),
//     );

//     // Get the first child of the root node (should be a heading node)
//     let heading = ast.first_child().unwrap();

//     // Convert the heading node to a paragraph node
//     let heading_data = heading.data.borrow_mut();
//     // heading_data.set_tag(Tag::Paragraph);
//     print!("{:?}", heading_data);


// }