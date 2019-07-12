use std::env;
use std::sync::{Arc, Mutex};
use hyper::Client;
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use dotenv::dotenv;
use url::Url;
use rand::seq::SliceRandom;

pub fn fetch_gifs() -> impl Future<Item=Vec<Gif>, Error=()> {
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

    let uri = url.into_string().parse().unwrap();
    println!("URI: {}", uri);

    fetch_json(uri)
        .and_then(|g| {
            let mut simple_gifs: Vec<Gif> = Vec::new();

            for gif in g.data.iter() {
                simple_gifs.push(Gif::new(gif));
            }

            Ok(simple_gifs)
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        })
}

fn fetch_json(url: hyper::Uri) -> impl Future<Item=GiphyResult, Error=FetchError> {
    let https = HttpsConnector::new(4).expect("TLS Initialization failed!");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
        .and_then(|res| {
            // asynchronously concatenate chunks of the body
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        // use the body after concatenation
        .and_then(|body| {
            // try to parse as json with serde_json
            let gifs_result = serde_json::from_slice(&body)?;

            Ok(gifs_result)
        })
        .from_err()
}

pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

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

impl Clone for Gif {
    fn clone(&self) -> Gif {
        Gif {
            id: self.id.clone(),
            slug: self.slug.clone(),
            title: self.slug.clone(),
            url: self.url.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GiphyResult {
    pub data: Vec<GiphyGif>
}

#[derive(Deserialize, Debug)]
pub struct GiphyGif {
    pub id: String,
    pub slug: String,
    pub url: String,
    pub title: String,
    pub images: Images,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub original: Image
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub width: String,
    pub height: String,
}

impl Clone for GiphyGif {
    fn clone(&self) -> GiphyGif {
        GiphyGif {
            id: self.id.clone(),
            slug: self.slug.clone(),
            url: self.url.clone(),
            title: self.title.clone(),
            images: self.images.clone(),
        }
    }
}

impl Clone for Images {
    fn clone (&self) -> Images {
        Images {
            original: self.original.clone(),
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Image {
        Image {
            url: self.url.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
}

/// This struct must implement `Clone` and `StateData` to be applicable
/// for use with the `StateMiddleware`, and be shared via `Middleware`.
#[derive(Clone, StateData)]
pub struct GifHolder {
    gifs: Arc<Mutex<Vec<Gif>>>,
}

impl GifHolder {
    pub fn new(gifs: Vec<Gif>) -> Self {
        Self {
            gifs: Arc::new(Mutex::new(gifs)),
        }
    }

    pub fn get_random_gif(&self) -> Option<Gif> {
        let gifs = self.gifs.lock().unwrap();
        let gif = gifs.choose(&mut rand::thread_rng());

        match gif {
            Some(g) => Some(g.clone()),
            None => None
        }
    }
}
