use gotham::state::{State};
use gotham::helpers::http::response::{create_empty_response, create_response};

use askama::Template;
use hyper::{Body, Response, StatusCode};

#[derive(Debug, Template)]
#[template(path = "iframe.html")]
pub struct Iframe {}

pub fn render_iframe(state: State) -> (State, Response<Body>) {

    let tpl = Iframe {};

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
