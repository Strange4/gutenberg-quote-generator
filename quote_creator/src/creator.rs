use lazy_static::lazy_static;
use std::io::Write;
use std::fs::File;
use nanorand::Rng;
use regex::Regex;
use std::sync::Arc;
use indicatif::ProgressBar;
use reqwest::Client;
use crate::gutendex::BookInfo;


pub fn book_to_quote(text: &String, number_of_quotes: u32) -> Vec<String> {
    let chars = text
        .chars()
        .collect::<Vec<_>>();
    (0..number_of_quotes).map(|_|{
        let length = chars.len() - 1;
        let r_number = nanorand::tls_rng().generate_range(0..length);
        let beginning = (r_number..length)
        .find(|index| {
            is_end_char(&chars[*index])
        }).unwrap_or(0) + 1;
        let end = (beginning..length)
        .find(|index| {
            is_end_char(&chars[*index])
        }).unwrap_or(length - 1) + 1;
        chars[beginning..end].iter().collect()
    }).collect()
}

pub fn is_end_char(character: &char) -> bool{
    match character {
        '.' => true,
        '!' => true,
        '?' => true,
        _ => false
    }
}


pub async fn get_book(book: BookInfo, client: Client, progress: Arc<ProgressBar>) -> Option<(String, String)>{
    let response = client.get(&book.guten_url)
    .send().await.unwrap();
    if !response.status().is_success(){
        return None;
    }
    let text = response.text()
        .await.unwrap();
    if let Some(book_text) = parse_book(&text){
        progress.inc(1);
        return Some((book_text, book.title));
    }
    None
}

pub fn parse_book(text: &String) -> Option<String> {
    lazy_static! {
        static ref START_SEPARATOR: Regex = Regex::new("\\*\\*\\* START OF TH.* \\*\\*\\*")
            .expect("the regex is not nice");
        static ref END_SEPARATOR: Regex = Regex::new("\\*\\*\\* END OF TH.* \\*\\*\\*").expect("this regex is invalid");
    }
    let lines: Vec<&str> = START_SEPARATOR.split(text)
        .collect();
    if lines.len() != 2{
        return None;
    }
    let beginning = lines[1];
    let book = END_SEPARATOR.split(beginning).collect::<Vec<_>>()[0];
    Some(book.trim().to_string())
}

pub fn write_quotes_to_file(title: &mut String, quotes: &Vec<String>, folder: &String){
    let mut path = folder.clone();
    path.push('/');
    path.push_str(&clean_file_name(title));
    path.push_str(".txt");
    let mut file = File::create(path).unwrap();
    let quotes = quotes.join("");
    file.write_all(quotes.as_bytes()).unwrap();
}

pub fn clean_file_name(title: &mut String) -> String{
    const ILLEGAL_CHARS: &[char] = &[
    '#','%','&','{','}','\\',
    '<','>','*','?','/','$',
    '!','\'','"',':','@','+',
    '`','|','=',
    ];

    title.truncate(200);
    title.replace(&ILLEGAL_CHARS[..], "")
}