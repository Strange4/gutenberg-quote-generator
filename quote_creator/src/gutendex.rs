use std::collections::HashMap;
use futures::{stream, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use crate::PARALLEL_REQUESTS;

pub async fn get_top_book_links(ammount: u32) -> Vec<BookInfo>{
    const GUTENDEX_URL: &str = "https://gutendex.com/books/?page=";
    const BOOKS_PER_PAGE: u32 = 32;
    let client = Client::new();
    let number_of_pages = (ammount / BOOKS_PER_PAGE) + 1;
    let results = stream::iter(1..=number_of_pages)
        .map(|index|{
            let client = client.clone();
            tokio::spawn(async move {
                let mut url = GUTENDEX_URL
                    .to_string();
                url.push_str(index.to_string().as_str());
                get_books(&url, &client).await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .collect::<Vec<_>>().await;
    
    let mut books = results.into_iter()
        .map(|result|{
            result.unwrap().results
        }).flatten()
        .map(|book| {
            match try_get_url(&book) {
                Some(url) => {Some(BookInfo { title: book.title, guten_url: url })},
                None => None
            }
        })
        .filter_map(|maybe_book|{ maybe_book })
        .collect::<Vec<_>>();
    books.truncate(ammount as usize);
    books
}

fn try_get_url(book: &GutenBook) -> Option<String>{
    const TXT_FORMATS: &[&str] = &[
        "text/plain", 
        "text/plain; charset=us-ascii",
        "text/plain; charset=utf-8"
        ];
    for format in TXT_FORMATS.into_iter(){
        if let Some(url) = book.formats.get(*format) {
            return Some(url.clone());
        }
    }
    None
}

async fn get_books(page_url: &String, client: &Client) -> GutenPage{
    client.get(page_url)
        .send()
        .await.unwrap()
        .json::<GutenPage>()
        .await.expect(&page_url)
}

#[derive(Deserialize)]
struct GutenPage{
    results: Vec<GutenBook>
}

#[derive(Deserialize)]
struct GutenBook{
    title: String,
    formats: HashMap<String, String>
}

#[derive(Debug)]
pub struct BookInfo{
    pub title: String,
    pub guten_url: String
}