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
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use futures::future::lazy;

use futures::{Future};

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;

mod routes;
use routes::simple_form;

mod giphy;
use giphy::{GifHolder, fetch_gifs};

fn main() {
    tokio::run(lazy(|| {
        fetch_gifs().and_then(|gifs| {
            let addr = "127.0.0.1:7878";
            println!("Listening for requests at http://{}", addr);

            // create the gifs holder to share across handlers
            let gifs_holder = GifHolder::new(gifs);

            gotham::start(addr, router(gifs_holder));

            Ok(())
        }).map_err(|e| {
            println!("Oh no! {:?}", e);
        })
    }));
}

fn router(gifs_holder: GifHolder) -> Router {
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
