use serde::{Deserialize};

use gotham::state::{State, FromState};
use gotham::helpers::http::response::{create_empty_response, create_response};

use askama::Template;
use hyper::{Body, Response, StatusCode};

use gif_service::redis::{get_random_gif};

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
    let gif = match get_random_gif() {
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
