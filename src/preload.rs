use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find_preload_links<P: AsRef<Path>>(path: P) -> Vec<String> {
    let re =
        Regex::new(r#"<link rel="preload" href="(?P<link>[/.\w]*)" as="?P<as>([\w]*)"/>"#).unwrap();
    let contents = fs::read_to_string(&path).unwrap();
    re.captures_iter(&contents)
        .map(|caps| get_link_header(&caps["link"], &caps["as"]))
        .collect()
}

fn get_link_header(link: &str, r#as: &str) -> String {
    format!("rel=preload; <{}>; as={};", link, r#as)
}
