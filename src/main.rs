extern crate gotham;
#[macro_use]
extern crate gotham_derive;

extern crate askama;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate mime;
extern crate dotenv;
extern crate url;
extern crate tokio;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::sync::{Arc, Mutex};

use futures::future::lazy;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;

mod routes;
use routes::simple_form;

mod giphy;
use giphy::{run_giphy, Gif, GiphyResult};

fn main() {
    tokio::run(lazy(|| {
        let addr = "127.0.0.1:7878";
        println!("Listening for requests at http://{}", addr);

        tokio::spawn(lazy(|| {
            run_giphy()
        }));

        // gotham::start_on_executor(addr, router, executor: TaskExecutor);
        gotham::start(addr, router());

        Ok(())
    }));
}

fn router() -> Router {
    // create the gifs holder to share across handlers
    let gifs_holder = GifHolder::new();

    // create our state middleware to share the gifs
    let middleware = StateMiddleware::new(gifs_holder);

    // create a middleware pipeline from our middleware
    let pipeline = single_middleware(middleware);

    // construct a basic chain from our pipeline
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
       route.get("/").to(simple_form);
   })
}

/// This struct must implement `Clone` and `StateData` to be applicable
/// for use with the `StateMiddleware`, and be shared via `Middleware`.
#[derive(Clone, StateData)]
pub struct GifHolder {
    gifs: Arc<Mutex<Vec<Gif>>>,
}

impl GifHolder {
    pub fn new() -> Self {
        Self {
            gifs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_gif(&self, gif: Gif) {
        let mut gifs = self.gifs.lock().unwrap();
        gifs.push(gif);
    }

    pub fn get_gif(&self, slug: &str) -> Option<Gif> {
        let gifs = self.gifs.lock().unwrap();
        let gif = gifs.iter().filter(|g| g.slug == slug).last();

        match gif {
            Some(g) => Some(g.clone()),
            None => None
        }
    }
}
