//! 


pub mod youtube; 

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// defined to parse a markdown file to html
extern crate comrak;
use comrak::{markdown_to_html, ComrakOptions};


use youtube::{ to_embed_tag as youtube_to_embed_tag };



// pub fn get_path(file_path: &str) -> String {
//     let path = Path::new(file_path);
//     let display = path.display();
//     display.to_string()
// }


/// 파일 타입을 확인합니다.
#[allow(dead_code)] 
fn file_type(file_path: &str) -> std::io::Result<()>{
    
  let metadata = fs::metadata(file_path)?;
  let file_type = metadata.file_type();
  println!("file_type: {:?}", file_type);

  if file_type.is_dir() {
      println!("{} is a directory", file_path);
  } else if file_type.is_file() {
      println!("{} is a file", file_path);
  } else {
      println!("{} is not a file or directory", file_path);
  }

  Ok(())
}

// pub fn file_open(file_path: &str)  {
//     let path = Path::new(file_path);
//     let display = path.display();
//     let file = File::open(&path);
//     match file {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => {
//             println!("file: {:?}", file);
//         },
//     };
// }


/// 파일을 엽니다.
pub fn file_open(file_path: &str) -> Result<File, std::io::Error> {
  let path = Path::new(file_path);
  // let display = path.display();
  let file = File::open(&path)?;
  Ok(file)
}


/// 파일을 읽습니다.
pub fn file_read(file: &mut File) -> Result<String, std::io::Error> {
  let mut s = String::new();
  file.read_to_string(&mut s)?;
  Ok(s)
}


/// 파일을 읽고 쓰기 위해서 엽니다.
pub fn open_read(path: &str) -> String  { 
  let mut file = file_open(path).unwrap();
  let s = file_read(&mut file).unwrap();
  // println!("{} contains:\n{}", path, s);
  s 
}




/// 마크다운 파일을 읽고 html로 변환합니다. 
/// **Example**
/// ```
/// let path = "README.md";
/// let html = md_to_html(path);
/// ```
pub fn md_to_html(path: &str) -> String {
  let mut file = file_open(path).unwrap();
  let s = file_read(&mut file).unwrap();
  markdown_to_html(&s, &ComrakOptions::default())
}


/// youtube 링크를 embed 태그로 변환합니다.
pub fn to_embed_tag(html: &str) -> String {
  youtube_to_embed_tag(html)
}
