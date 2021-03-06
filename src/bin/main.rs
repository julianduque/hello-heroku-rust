use hello_heroku_rust::ThreadPool;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream) {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let stage = b"GET /stage HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_millis(500));
        ("HTTP/1.1 200 OK \r\n\r\n", "index.html")
    } else if buffer.starts_with(stage) {
        ("HTTP/1.1 200 OK \r\n\r\n", app_env.as_str())
    } else {
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "404.html")
    };

    let contents = match filename {
        "index.html" | "404.html" => fs::read_to_string(filename).unwrap(),
        _ => String::from(filename),
    };

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
