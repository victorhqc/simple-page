extern crate gif_service;

use gotham::router::builder::*;
use gotham::router::Router;
use dotenv::dotenv;

mod helpers;
use helpers::get_server_address;

mod routes;
use routes::simple_form;
use routes::render_iframe;

fn main() {
    dotenv().ok();

    let addr = get_server_address();
    println!("Listening for requests at http://{}", addr);

    gotham::start(addr, router());
}

fn router() -> Router {
    build_simple_router(|route| {
       route.get("/").to(simple_form);
       route.get("/embedded").to(render_iframe);
   })
}
