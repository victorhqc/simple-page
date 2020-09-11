use dotenv::dotenv;
use futures::Future;
use rand::seq::SliceRandom;
use std::env;
use url::Url;

mod utils;
pub use self::utils::*;

#[derive(Debug, Clone)]
pub struct Giphy {
    pub gifs: Vec<Gif>,
}

impl Giphy {
    pub fn fetch(needle: String) -> impl Future<Item = Vec<Gif>, Error = ()> {
        dotenv().ok();
        let giphy_key = env::var("GIPHY_KEY").expect("GIPHY_KEY must be set in your .env file");
        let url = Url::parse_with_params(
            "https://api.giphy.com/v1/gifs/search",
            &[("api_key", giphy_key), ("q", needle)],
        )
        .expect("Can't parse url");
        let uri = url.into_string().parse().unwrap();
        println!("URI: {}", uri);
        fetch_giphy_json(uri)
            .and_then(|g| {
                let mut simple_gifs: Vec<Gif> = Vec::new();
                for gif in g.data.iter() {
                    simple_gifs.push(Gif::new(gif));
                }

                Ok(simple_gifs)
            })
            // if there was an error print it
            .map_err(|e| match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            })
    }

    pub fn store_gifs(&mut self, gifs: Vec<Gif>) {
        self.gifs = gifs;
    }

    pub fn get_random(&self) -> Option<String> {
        let gif = self.gifs.choose(&mut rand::thread_rng());
        match gif {
            Some(g) => Some(g.url.clone()),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gif {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub url: String,
}

impl Gif {
    pub fn new(gif: &GiphyGif) -> Self {
        Self {
            id: gif.id.clone(),
            slug: gif.slug.clone(),
            title: gif.slug.clone(),
            url: gif.images.original.url.clone(),
        }
    }
}
