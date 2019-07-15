#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use futures::future::lazy;
use std::{thread, time};

use futures::{Future};

mod giphy;
use giphy::{fetch_gifs};

use rand::seq::SliceRandom;

fn main() {

    loop {
        tokio::run(lazy(move || {
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

            fetch_gifs(word).and_then(|_gifs| {
                println!("Gifs!");
                Ok(())
            }).map_err(|e| {
                println!("Oh no! {:?}", e);
            })
        }));

        let five_mins = time::Duration::from_secs(60 * 5);
        thread::sleep(five_mins);
    }
}

fn get_random_word(words: Vec<String>) -> Option<String> {
    let word = words.choose(&mut rand::thread_rng());

    match word {
        Some(w) => Some(w.to_string()),
        None => None
    }
}
