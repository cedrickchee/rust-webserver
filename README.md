# Simple Web Server

A multi-threaded web server build with [Rust](https://www.rust-lang.org/) 2018 edition.

**Intention of this repo**

An **educational project** for learning Rust programming language. The method used is not the best way to build a production-ready web server with Rust.

## Building a Web Server

I've written the basic HTTP server and thread pool manually so you can learn the general ideas and techniques behind the crates you might use in the future. HTTP requests are serve using the thread pool.

We’ll work with the raw bytes of TCP and HTTP requests and responses.

## Project Status

Although this is an educational project, I have plan to continue enhancing this project, and here are some ideas:
- More robust error handling.
- Add tests of the library's functionality.
- Improve API design.
- Stretch goal: implement a single-threaded non-blocking I/O server using async-await on Rust 1.39.0.

## Demo

- _TODO: record terminal session and embed it here._
