use hyper::Client;
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};

pub fn fetch_giphy_json(url: hyper::Uri) -> impl Future<Item=GiphyResult, Error=FetchError> {
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
