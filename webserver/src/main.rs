#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate lazy_static;

mod giphy;
mod helpers;
mod routes;

use crate::giphy::Giphy;
use dotenv::dotenv;
use futures::Future;
use giphy_utils::giphy::Giphy as GiphyUtil;
use gotham::router::builder::*;
use gotham::router::Router;
use helpers::get_server_address;
use rand::seq::SliceRandom;
use routes::render_iframe;
use routes::simple_form;
use routes::{render_page, PageQueryExtractor};
use routes::{render_page_with_iframe, PageWithIframeQueryExtractor};
use std::{thread, time};

lazy_static! {
    pub static ref GIPHY: Giphy = Giphy::new();
}

fn main() {
    dotenv().ok();
    start_giphy();

    let addr = get_server_address();
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router());
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(simple_form);
        route.get("/embedded").to(render_iframe);
        route
            .get("/page_with_iframe")
            .with_query_string_extractor::<PageWithIframeQueryExtractor>()
            .to(render_page_with_iframe);
        route
            .get("/page")
            .with_query_string_extractor::<PageQueryExtractor>()
            .to(render_page);
    })
}

fn start_giphy() {
    thread::spawn(|| loop {
        tokio::run(gif_service());

        let five_mins = time::Duration::from_secs(60 * 5);
        thread::sleep(five_mins);
    });
}

fn gif_service() -> impl Future<Item = (), Error = ()> {
    let words = vec![
        String::from("pug"),
        String::from("cat"),
        String::from("llama"),
    ];

    let word = get_random_word(words);
    let word = match word {
        Some(word) => word,
        None => panic!("No random word!"),
    };

    println!("Fetching gifs for {}", word);
    GiphyUtil::fetch(word)
        .and_then(|gifs| {
            GIPHY.update(gifs).unwrap();
            Ok(())
        })
        .map_err(|e| {
            println!("Oh no! {:?}", e);
        })
}

fn get_random_word(words: Vec<String>) -> Option<String> {
    let word = words.choose(&mut rand::thread_rng());

    match word {
        Some(w) => Some(w.to_string()),
        None => None,
    }
}
