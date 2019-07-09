use std::env;
use std::io::{self, Write};

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

use dotenv::dotenv;

use url::Url;

pub fn run_giphy(gifs: &mut Gifs) {
    dotenv().ok();

    let giphy_key = env::var("GIPHY_KEY").expect("GIPHY_KEY must be set in your .env file");

    let url = Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/search",
        &[
            ("api_key", giphy_key),
            ("q", "pug".to_string())
        ]
    )
    .expect("Can't parse url");

    let https = HttpsConnector::new(4).expect("TLS Initialization failed!");

    rt::run(rt::lazy(|| {
        let client = Client::builder()
            .build::<_, hyper::Body>(https);
        let uri = url.into_string().parse().unwrap();
        println!("URI: {}", uri);

        client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());

                res
                    .into_body()
                    .for_each(|chunk| {
                        io::stdout()
                            .write_all(&chunk)
                            .map_err(|e| {
                                panic!("Something went terribly wrong {}", e)
                            })
                    })
            })
            .map_err(|err| {
                println!("Error: {}", err)
            })
    }));
}

pub struct Gif {
    name: String,
    url: String,
}

impl Gif {
    pub fn new(name: String, url: String) -> Gif {
        Gif { name, url }
    }
}

pub struct Gifs {
    imgs: Vec<Gif>,
}

impl Gifs {
    pub fn new() -> Gifs {
        Gifs {
            imgs: vec![],
        }
    }

    pub fn set_picture(&mut self, gif: Gif) {
        self.imgs.push(gif);
    }

    pub fn get_picture(&self, name: &str) -> Option<&Gif> {
        let img = self.imgs
            .iter()
            .filter(|g| {
                g.name == name
            })
            .last();

        match img {
            Some(i) => Some(i),
            None => None
        }
    }
}
