use rand::seq::SliceRandom;
use redis;
use redis::Commands;

use std::env;
use dotenv::dotenv;

use crate::giphy::{Gif};

pub fn save_in_redis(gifs: Vec<Gif>) {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in your .env file");
    let client = redis::Client::open(redis_url.as_str()).unwrap();
    let con = client.get_connection().unwrap();

    let _: () = con.set("gif_len", gifs.len()).unwrap();

    for (i, gif) in gifs.iter().enumerate() {
        let key = String::from(format!("gif:{}", i));
        let _: () = con.set(key.as_str(), gif.url.as_str()).unwrap();
    }
}

pub fn get_random_gif() -> Option<String> {
    let gifs = get_from_redis();
    let gif = gifs.choose(&mut rand::thread_rng());

    match gif {
        Some(g) => Some(g.clone()),
        None => None
    }
}

pub fn get_from_redis() -> Vec<String> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in your .env file");
    let client = redis::Client::open(
        redis_url.as_str()
    ).expect(format!("Can't open client {}", redis_url).as_str());
    let con = client.get_connection().expect("Can't get connection");

    let len: Option<String> = con.get("gif_len").unwrap();
    let len = match len {
        Some(l) => l,
        None => panic!("no len!"),
    };

    let len: usize = len.parse().unwrap();
    let mut gifs = Vec::with_capacity(len);

    for i in 0..len {
        let key = String::from(format!("gif:{}", i));
        let url: Option<String> = con.get(key.as_str()).unwrap();
        match url {
            Some(u) => gifs.push(u),
            None => {}
        };
    }

    gifs
}
