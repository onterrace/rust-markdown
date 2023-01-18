//! markdown 파일을 읽고 html로 변환합니다. 
//! 

#![warn(unused_variables)]
#![warn(dead_code)]

extern crate regex;
extern crate jirepos_markdown; 


use std::env;
use jirepos_markdown:: { md_to_html, to_embed_tag };


/// 마크다운 파일을 읽고 html로 렌더링합니다. 
/// 사용법은 다음과 같습니다.
/// 
/// ```shell 
/// ./jirepose_markdown ./data.md 
/// ```
/// 
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count = env::args().count();
    println!("arg_count: {}", arg_count);
    println!("args: {:?}", args);

    if arg_count < 2    {
        println!("파일 경로를 입력해주세요.");
        return;
    }
        
    let html = md_to_html(&args[1]);
    println!("html: {}", html);
    let last_html = to_embed_tag(&html);
    println!("last_html: {}", last_html);

}

