use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

use gif_service::giphy::{GiphyGif};

pub struct Gif {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub url: String,
}

impl Gif {
    pub fn new(gif: &GiphyGif) -> Self {
        Self {
            id: gif.id.clone(),
            slug: gif.slug.clone(),
            title: gif.slug.clone(),
            url: gif.images.original.url.clone(),
        }
    }
}

impl Clone for Gif {
    fn clone(&self) -> Gif {
        Gif {
            id: self.id.clone(),
            slug: self.slug.clone(),
            title: self.slug.clone(),
            url: self.url.clone(),
        }
    }
}

/// This struct must implement `Clone` and `StateData` to be applicable
/// for use with the `StateMiddleware`, and be shared via `Middleware`.
#[derive(Clone, StateData)]
pub struct GifHolder {
    gifs: Arc<Mutex<Vec<Gif>>>,
}

impl GifHolder {
    pub fn new() -> Self {
        Self {
            gifs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_random_gif(&self) -> Option<Gif> {
        let gifs = self.gifs.lock().unwrap();
        let gif = gifs.choose(&mut rand::thread_rng());

        match gif {
            Some(g) => Some(g.clone()),
            None => None
        }
    }
}
