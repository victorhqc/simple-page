use crossbeam::sync::ShardedLock;
use giphy_utils::giphy::{Gif, Giphy as GiphyUtil};

#[derive(StateData)]
pub struct Giphy {
  pub inner: ShardedLock<GiphyUtil>,
}

impl Giphy {
  pub fn new() -> Self {
    Self {
      inner: ShardedLock::new(GiphyUtil { gifs: Vec::new() }),
    }
  }

  pub fn get_inner(&self) -> Result<GiphyUtil, ()> {
    let giphy = self.inner.read().unwrap();
    let giphy = giphy.clone();
    Ok(giphy)
  }

  pub fn update(&self, gifs: Vec<Gif>) -> Result<(), ()> {
    let mut giphy = self.inner.write().unwrap();
    giphy.store_gifs(gifs);

    Ok(())
  }
}
