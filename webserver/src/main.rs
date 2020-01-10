extern crate gif_service;

use gotham::router::builder::*;
use gotham::router::Router;
use dotenv::dotenv;

#[macro_use]
extern crate gotham_derive;

mod helpers;
use helpers::get_server_address;

mod routes;
use routes::simple_form;
use routes::render_iframe;
use routes::{render_page, PageQueryExtractor};
use routes::{render_page_with_iframe, PageWithIframeQueryExtractor};

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
       route
        .get("/page_with_iframe")
        .with_query_string_extractor::<PageWithIframeQueryExtractor>()
        .to(render_page_with_iframe);
       route
        .get("/page")
        .with_query_string_extractor::<PageQueryExtractor>()
        .to(render_page);
   })
}
