extern crate gotham;
extern crate askama;
extern crate hyper;
extern crate mime;

use gotham::router::builder::*;
use gotham::router::Router;

mod routes;
use routes::simple_form;

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(simple_form);
    })
}
