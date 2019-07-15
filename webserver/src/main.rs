#[macro_use]
extern crate gotham_derive;

extern crate gif_service;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;

mod routes;
use routes::simple_form;

mod giphy;
use giphy::{GifHolder};

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    // create the gifs holder to share across handlers
    let gifs_holder = GifHolder::new();

    gotham::start(addr, router(gifs_holder));
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
