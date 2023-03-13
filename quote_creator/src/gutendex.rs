use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

pub async fn get_top_book_links(ammount: u32) -> Vec<BookInfo>{
    const GUTENDEX_URL: &str = "https://gutendex.com/books/";
    let client = Client::new();
    let page = get_books(&GUTENDEX_URL.to_string(), &client).await;
    let mut books = page.results;
    while (books.len() as u32) < ammount {
        match &page.next {
            None => break,
            Some(url) => {
                let mut page = get_books(url, &client).await;
                books.append(&mut page.results);
            }
        }
    }
    let mut books = books.into_iter().map(|book| {
        let url = try_get_url(&book);
        BookInfo { title: book.title, guten_url: url }
    }).collect::<Vec<_>>();
    books.truncate(ammount as usize);
    books
}

fn try_get_url(book: &GutenBook) -> String{
    const TXT_FORMATS: &[&str] = &[
        "text/plain", 
        "text/plain; charset=us-ascii",
        "text/plain; charset=utf-8"
        ];
    for format in TXT_FORMATS.into_iter(){
        if let Some(url) = book.formats.get(*format) {
            return url.clone();
        }
    }
    panic!("there are no formats that match");
}

async fn get_books(page_url: &String, client: &Client) -> GutenPage{
    client.get(page_url)
        .send()
        .await.unwrap()
        .json::<GutenPage>()
        .await.unwrap()
}

#[derive(Deserialize)]
struct GutenPage{
    next: Option<String>,
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