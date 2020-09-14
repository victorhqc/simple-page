# simple-page

## About

Nothing really useful. Just me learining about [gotham](https://gotham.rs/),
[hyper](https://hyper.rs/), [tokio](https://tokio.rs/) and Async programming in Rust in general.
I'm pretty new to [Futures](https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html#the-future-is-here)
but it's super interesting.

Watch [this video](https://www.youtube.com/watch?v=j0SIcN-Y-LA) to understand how it all works.

## Idea

I just want to have a **very simple static HTML** that renders some random gifs from giphy. I would
like to update those gifs every 5 minutes and just store them in memory for each request.

![idea](./idea.svg)

## Requirements

- Rust >= 1.36

## How to run

```
ADDRESS=127.0.0.1:7879 cargo run
```

1. Start webserver

```
cargo run
```

## How to run (nested iframes)

1. Compile production build

```bash
cargo build --release
```

1. Run start script

```bash
./scripts/start.sh
```
