extern crate gotham;
extern crate askama;
extern crate hyper;
extern crate mime;

use gotham::state::{State};
use gotham::helpers::http::response::{create_empty_response, create_response};

use askama::Template;
use hyper::{Body, Response, StatusCode};

use gif_service::redis::{get_random_gif};

const MESSAGE: &str = "Gotham";

#[derive(Debug, Template)]
#[template(path = "simple_form.html")]
pub struct SimpleForm {
    pub world: String,
    pub gif: String,
}

pub fn simple_form(state: State) -> (State, Response<Body>) {
    let gif = match get_random_gif() {
        Some(g) => g,
        None => String::from(""),
    };

    let tpl = SimpleForm {
        world: MESSAGE.to_string(),
        gif,
    };

    let res = match tpl.render() {
        Ok(content) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, res)
}
