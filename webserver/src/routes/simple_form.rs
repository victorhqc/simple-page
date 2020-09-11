use crate::helpers::get_iframe_address;
use crate::GIPHY;
use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::State;
use hyper::{Body, Response, StatusCode};

const MESSAGE: &str = "Gotham";

#[derive(Debug, Template)]
#[template(path = "simple_form.html")]
pub struct SimpleForm {
    pub world: String,
    pub gif: String,
    pub iframe_address: String,
}

pub fn simple_form(state: State) -> (State, Response<Body>) {
    let giphy = GIPHY.get_inner().unwrap();
    let gif = match giphy.get_random() {
        Some(g) => g,
        None => String::from(""),
    };

    let tpl = SimpleForm {
        world: MESSAGE.to_string(),
        gif,
        iframe_address: get_iframe_address(),
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
