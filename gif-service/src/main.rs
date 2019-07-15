use std::{thread, time};

use futures::{Future};
use rand::seq::SliceRandom;


mod giphy;
use giphy::{fetch_gifs};

mod redis;
use crate::redis::{save_in_redis};

fn main() {
    loop {
        tokio::run(gif_service());

        let five_mins = time::Duration::from_secs(60 * 5);
        thread::sleep(five_mins);
    }
}

fn gif_service() -> impl Future<Item=(), Error=()> {
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
    fetch_gifs(word).and_then(|gifs| {
        println!("Obtained {} new gifs", gifs.len());
        save_in_redis(gifs);
        Ok(())
    }).map_err(|e| {
        println!("Oh no! {:?}", e);
    })
}

fn get_random_word(words: Vec<String>) -> Option<String> {
    let word = words.choose(&mut rand::thread_rng());

    match word {
        Some(w) => Some(w.to_string()),
        None => None
    }
}
