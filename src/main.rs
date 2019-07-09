extern crate gotham;
extern crate askama;
extern crate hyper;
extern crate hyper_tls;
extern crate mime;
extern crate dotenv;
extern crate url;

use gotham::router::builder::*;
use gotham::router::Router;

mod routes;
use routes::simple_form;

mod giphy;
use giphy::{run_giphy, Gifs};

fn main() {
    let mut gifs = Gifs::new();

    run_giphy(&mut gifs);

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router(&gifs))
}

fn router(gifs: &Gifs) -> Router {
    build_simple_router(|route| {
        route.get("/").to(simple_form);
    })
}
