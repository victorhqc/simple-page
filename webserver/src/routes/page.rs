use serde::Deserialize;

use crate::GIPHY;
use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};

#[derive(Debug, Template)]
#[template(path = "page.html")]
pub struct Page {
    pub title: String,
    pub gif: String,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct PageQueryExtractor {
    title: String,
}

pub fn render_page(mut state: State) -> (State, Response<Body>) {
    let giphy = GIPHY.get_inner().unwrap();
    let gif = match giphy.get_random() {
        Some(g) => g,
        None => String::from(""),
    };

    let query_param = PageQueryExtractor::take_from(&mut state);

    let tpl = Page {
        title: query_param.title,
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
