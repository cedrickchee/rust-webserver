use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use webserver::ThreadPool;

fn main() {
    // Because we’re writing a basic server just for learning purposes,
    // we won’t worry about handling these kinds of errors; instead,
    // we use unwrap to stop the program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The incoming method on TcpListener returns an iterator that gives us
    // a sequence of streams (more specifically, streams of type TcpStream).
    // A single stream represents an open connection between the client and the server.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // A connection is the name for the full request and response process
        // in which a client connects to the server, the server generates a response,
        // and the server closes the connection.
        // println!("Connection established");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing until all the bytes
    // are written to the connection; TcpStream contains an internal buffer to
    // minimize calls to the underlying operating system.
    stream.flush().unwrap();
}
