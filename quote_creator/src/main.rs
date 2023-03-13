mod gutendex;

use futures::{stream, StreamExt};
use gutendex::BookInfo;
use reqwest::Client;
use std::{fs::File, time::Duration};
use lazy_static::lazy_static;
use std::io::{Error, Write};
use regex::Regex;
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{theme::ColorfulTheme, Input};

const PARALLEL_REQUESTS: usize = 10;
const BOOKS_FOLDER: &str = "../books_scrapped";
const BOOK_AMMOUNT: u32 = 100;

#[tokio::main]
async fn main() {
    let books_to_download = get_number_of_books();
    let folder = get_folder();

    
    let spinner = create_spinner("Getting links".to_string());
    let books = tokio::task::spawn_blocking(
        move || gutendex::get_top_book_links(books_to_download))
        .await.unwrap();
    spinner.finish();
    println!("Getting the books...");
    let progress = Arc::new(
        create_progress_bar(books_to_download));
    let client = Client::new();
    let bodies = stream::iter(books)
        .map(|book| {
            let client = client.clone();
            let progress = progress.clone();
            let folder = folder.clone();
            tokio::spawn(async {
                get_book(book, client, progress, folder).await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .collect::<Vec<_>>().await;
    progress.finish();
    // these files will be used later so they will be used to create the quotes
    let files = bodies.into_iter()
        .map(|handle| {
            handle.unwrap()
        })
        .filter_map(|handle|{
            handle
        }).collect::<Vec<_>>();
    
}

async fn get_book(book: BookInfo, client: Client, progress: Arc<ProgressBar>, save_folder: String) -> Option<File>{
    let response = client.get(&book.guten_url)
    .send().await.unwrap();
    progress.inc(1);
    if response.status().is_success() {
        let text = response.text()
            .await.unwrap();
        let book_text = parse_book(text.as_str());
        let file = write_book_to_file(&book.title, book_text, save_folder)
            .unwrap();
        progress.inc(1);
        return Some(file);
    }
    None::<File>
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

fn write_book_to_file(title: &String, book: &str, mut folder: String) -> Result<File, Error>{
    folder.push('/');
    folder.push_str(&clean_file_name(title));
    folder.push_str(".txt");
    let mut file = File::create(folder)?;
    file.write_all(book.as_bytes())?;
    // why do i have to flush???
    // file.flush().await?;
    Ok(file)
}

fn clean_file_name(title: &String) -> String{
    const ILLEGAL_CHARS: &[char] = &[
        '<', '>', '*', '?', '/',
        '"', ':', '|', '\\',
    ];

    title.replace(&ILLEGAL_CHARS[..], "")
}

fn create_progress_bar(books_to_download: u32) -> ProgressBar {
    let progress = ProgressBar::new(books_to_download as u64 * 2);
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


fn get_number_of_books() -> u32{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the number of books to download")
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
