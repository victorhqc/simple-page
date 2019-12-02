use std::env;

pub fn get_server_address() -> String {
    env::var("ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:7878".into())
        .parse()
        .expect("Can't parse ADDRESS variable")
}

pub fn get_iframe_address() -> String {
    env::var("IFRAME_ADDRESS")
        .unwrap_or_else(|_| "http://127.0.0.1:7879/embedded".into())
        .parse()
        .expect("Can't parse ADDRESS variable")
}
