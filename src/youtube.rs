//! 정규식을 이용하여 Youtube URL을 처리한다.
//! 
//! \<a\> tag 만을 처리한다는 것에 주의 하세요. 다음과 같은 URL은 처리하지 않습니다. 
//! ```shll
//! https://www.youtube.com/watch?v=1Q8fG0TtVAY
//! ```
//! 

use regex::Regex;


/// Youtubue URL을 capture하기 위한 정규식
pub const REGEX_YOUTUBE_URL: &str = r#"<a[^>]*((youtu.be/)|(watch\?))\??v?=?(?P<video_code>[^#\?]*)">.*?</a>"#;


/// youtube url인지 확인한다.
pub fn is_youtube_url(a_tag: &str) -> bool {
  let re = Regex::new(REGEX_YOUTUBE_URL).unwrap();
  re.is_match(a_tag)
}


/// Youtube URL에서 video id를 추출한다.
pub fn extract_video_id(url: &str) -> &str {
  let re = Regex::new(REGEX_YOUTUBE_URL).unwrap();
  let captures = re.captures(url).unwrap();
  captures.get(4).unwrap().as_str()
}

/// Youtube URL을 embed tag로 변환한다.
pub fn to_embed_tag(a_tag: &str) -> String {
  let re = Regex::new(REGEX_YOUTUBE_URL).unwrap();
  let embed_tag = r#"<iframe width="600" height="400" src="https://www.youtube.com/embed/$video_code" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>"#;
  let replaced_str = re.replace_all(a_tag, embed_tag);
  replaced_str.to_string()
}


