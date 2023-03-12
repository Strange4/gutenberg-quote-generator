use std::collections::HashMap;

use reqwest::{self, blocking, Error};
use serde::Deserialize;

pub fn get_top_book_links(ammount: u32) -> Vec<BookInfo>{
    const GUTENDEX_URL: &str = "https://gutendex.com/books/";
    let page = get_books(&GUTENDEX_URL.to_string()).unwrap();
    let mut books = page.results;
    // man i wish if left chains were a thing
    while (books.len() as u32) < ammount {
        match &page.next {
            None => break,
            Some(url) => {
                let mut page = get_books(url)
                    .unwrap();
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

fn get_books(page_url: &String) -> Result<GutenPage, Error>{
    let page = blocking::get(page_url)?
        .json::<GutenPage>()?;
    Ok(page)
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