mod gutendex;

use futures::{stream, StreamExt};
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use lazy_static::lazy_static;
use std::io::Error;
use regex::Regex;

const PARALLEL_REQUESTS: usize = 10;
const BOOKS_FOLDER: &str = "../books_scrapped/file.txt";

#[tokio::main]
async fn main() {
    let books = tokio::task::spawn_blocking(
        || gutendex::get_top_book_links(100))
        .await.unwrap();

    let client = Client::new();
    let bodies = stream::iter(books)
        .map(|book| {
            let client = client.clone();
            tokio::spawn(async move {
                let response = client.get(book.guten_url)
                    .send().await.unwrap();
                if response.status().is_success() {
                    let text = response.text()
                        .await.unwrap();
                    let book_text = parse_book(text.as_str());
                    let file = write_book_to_file(&book.title, book_text)
                        .await.unwrap();
                    return Some(file);
                }
                None::<File>
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .collect::<Vec<_>>().await;

    // these files will be used later so they will be used to create the quotes
    let files = bodies.into_iter()
        .map(|handle| {
            handle.unwrap()
        })
        .filter_map(|handle|{
            handle
        }).collect::<Vec<_>>();
    
}

fn parse_book(text: &str) -> &str {
    lazy_static! {
        static ref START_SEPARATOR: Regex = Regex::new("\\*\\*\\* START OF TH.* \\*\\*\\*")
            .expect("the regex is not nice");
        static ref END_SEPARATOR: Regex = Regex::new("\\*\\*\\* END OF TH.* \\*\\*\\*").expect("this regex is invalid");
    }
    let lines: Vec<&str> = START_SEPARATOR.split(text)
        .collect();
    let beginning = lines[1];
    let book = END_SEPARATOR.split(beginning).collect::<Vec<_>>()[0];
    book.trim()
}

async fn write_book_to_file(title: &String, book: &str) -> Result<File, Error>{
    let file_path = BOOKS_FOLDER
        .replace("file", &clean_file_name(title));
    let mut file = File::create(file_path).await?;
    println!("Writing book: {}", title);
    file.write_all(book.as_bytes()).await?;
    // why do i have to flush???
    file.flush().await?;
    Ok(file)
}

fn clean_file_name(title: &String) -> String{
    const ILLEGAL_CHARS: &[char] = &[
        '<', '>', '*', '?', '/',
        '"', ':', '|', '\\',
    ];

    title.replace(&ILLEGAL_CHARS[..], "")
}