mod gutendex;

use futures::{stream, StreamExt};
use gutendex::BookInfo;
use rayon::prelude::*;
use reqwest::Client;
use std::{fs::File, time::Duration};
use lazy_static::lazy_static;
use std::io::Write;
use nanorand::Rng;
use regex::Regex;
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{theme::ColorfulTheme, Input, Confirm};

const PARALLEL_REQUESTS: usize = 10;
const BOOKS_FOLDER: &str = "../quotes_scrapped";
const BOOK_AMMOUNT: u32 = 100;

#[tokio::main]
async fn main() {
    let books_to_download = get_number_from_user("Number of books to download?");
    let number_of_quotes = get_number_from_user("Number of quotes per book?");
    let folder = get_folder();


    
    let spinner = create_spinner("Getting links".to_string());
    let books = gutendex::get_top_book_links(books_to_download).await;
    spinner.finish();
    println!("Getting the books...");
    let progress = Arc::new(
        create_progress_bar(books_to_download));
    let client = Client::new();
    let bodies = stream::iter(books)
        .map(|book| {
            let client = client.clone();
            let progress = progress.clone();
            tokio::spawn(async {
                get_book(book, client, progress).await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .collect::<Vec<_>>().await;
    progress.finish();
    let book_and_titles = bodies.into_iter()
        .filter_map(|handle|{
            handle.unwrap()
        }).collect::<Vec<_>>();
    let mut done = false;
    while !done {
        let spinner = create_spinner("Creating quotes".to_string());
        book_and_titles.par_iter()
            .for_each(|(text, title)| {
                let quotes = book_to_quote(text, number_of_quotes);
                write_quotes_to_file(title, &quotes, &folder);
            });
        spinner.finish();
        done = !confirm_dialog("Do you want to regenerate the quotes?");
    }
}

fn book_to_quote(text: &String, number_of_quotes: u32) -> Vec<String> {
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

fn is_end_char(character: &char) -> bool{
    match character {
        '.' => true,
        '!' => true,
        '?' => true,
        _ => false
    }
}


async fn get_book(book: BookInfo, client: Client, progress: Arc<ProgressBar>) -> Option<(String, String)>{
    let response = client.get(&book.guten_url)
    .send().await.unwrap();
    if !response.status().is_success(){
        return None;
    }
    let text = response.text()
        .await.unwrap();
    let book_text = parse_book(&text);
    progress.inc(1);
    Some((book_text, book.title))
}

fn parse_book(text: &String) -> String {
    lazy_static! {
        static ref START_SEPARATOR: Regex = Regex::new("\\*\\*\\* START OF TH.* \\*\\*\\*")
            .expect("the regex is not nice");
        static ref END_SEPARATOR: Regex = Regex::new("\\*\\*\\* END OF TH.* \\*\\*\\*").expect("this regex is invalid");
    }
    let lines: Vec<&str> = START_SEPARATOR.split(text)
        .collect();
    let beginning = lines[1];
    let book = END_SEPARATOR.split(beginning).collect::<Vec<_>>()[0];
    book.trim().to_string()
}

fn write_quotes_to_file(title: &String, quotes: &Vec<String>, folder: &String){
    let mut path = folder.clone();
    path.push('/');
    path.push_str(&clean_file_name(title));
    path.push_str(".txt");
    let mut file = File::create(path).unwrap();
    let quotes = quotes.join("");
    file.write_all(quotes.as_bytes()).unwrap();
}

fn clean_file_name(title: &String) -> String{
    const ILLEGAL_CHARS: &[char] = &[
        '<', '>', '*', '?', '/',
        '"', ':', '|', '\\',
    ];

    title.replace(&ILLEGAL_CHARS[..], "")
}

fn create_progress_bar(books_to_download: u32) -> ProgressBar {
    let progress = ProgressBar::new(books_to_download as u64);
    progress.enable_steady_tick(Duration::from_millis(120));
    progress.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
        ).unwrap()
        .tick_strings(&[
            "▹▹▹▹▹",
            "▸▹▹▹▹",
            "▹▸▹▹▹",
            "▹▹▸▹▹",
            "▹▹▹▸▹",
            "▹▹▹▹▸",
            "▪▪▪▪▪",
        ])
    );
    progress
}

fn create_spinner(message: String) -> ProgressBar{
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "◜",
                "◠",
                "◝",
                "◞",
                "◡",
                "◟",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(message);
    pb
}

fn confirm_dialog(message: &str) -> bool{
    Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt(message)
    .default(false)
    .interact()
    .unwrap()
}

fn get_number_from_user(message: &str) -> u32{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(BOOK_AMMOUNT)
        .interact_text()
        .unwrap()
}

fn get_folder() -> String{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("output folder?")
        .default(BOOKS_FOLDER.to_string())
        .interact_text()
        .unwrap()
}
