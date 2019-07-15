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

## Workaround solution

Unfortunately, my Rust skills are quite bad at this point and I've come with all sorts of problems
trying to store my gifs somewhere where the multithreaded gotham server can access them. For
example, the places where I want to store them either doesn't live long enough or I just break my
head against the borrow checker.

As a temporary solution, I'm using Redis just to store the gif URLS and then read them in each
request. This has the nice solution of avoid dealing with async problems, memory problems, etc.
**But it requires of having two rust instances running** plus the Redis server. So it stops being
a simple page.

![workaround](./workaround.svg)

## Requirements

- Docker
- Rust >= 1.36

## How to run


1. Start Redis server
```sh
docker run -d -p 6379:6379 redis
```

2. Start gif service
```sh
cargo run --bin gif-service
```

3. Start webserver
```
cargo run --bin webserver
```
