use std::env;

use hyper::Client;
// use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
// use tokio::spawn;

// use futures::future::map::Map;

use dotenv::dotenv;

use url::Url;

pub fn run_giphy() -> impl Future<Item=(), Error=()> {
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
        // use the parsed vector
        // .map(|gifs_result| {
        //     // print users
        //     println!("gifs: {:#?}", gifs_result);
        //
        //     // let b = gifs.as_mut();
        //     //
        //     // for gif in gifs_result.data {
        //     //     b.push(gif.clone())
        //     // }
        //     // gifs.push(gifs_result.data.clone());
        // })
        .and_then(|_g| {
            Ok(())
        })
        // if there was an error print it
        .map_err(|e| {
            match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            }
        })

    // Run the runtime with the future trying to fetch, parse and print json.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    // rt::spawn(fut);
    // spawn(fut);
}

fn fetch_json(url: hyper::Uri) -> impl Future<Item=GiphyResult, Error=FetchError> {
    let https = HttpsConnector::new(4).expect("TLS Initialization failed!");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    println!("Fetch JSON");

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
            println!("Done!");

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

#[derive(Deserialize, Debug)]
pub struct GiphyResult {
    data: Vec<Gif>
}

#[derive(Deserialize, Debug)]
pub struct Gif {
    pub id: String,
    pub slug: String,
    pub url: String,
    pub title: String,
}

impl Clone for Gif {
    fn clone(&self) -> Gif {
        Gif {
            id: self.id.clone(),
            slug: self.slug.clone(),
            url: self.url.clone(),
            title: self.title.clone(),
        }
    }
}
