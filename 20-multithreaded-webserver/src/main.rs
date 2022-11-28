use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use webserver::threadpool::ThreadPool;

fn handle_request(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);

    let request_line = buf
        .lines()
        .next()  // Optional<Result<String, _>> since iter might return NULL
        .unwrap()
        .unwrap();
    
    let (status_code, html_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("200 OK", "index.html"),
        "GET /busybox HTTP/1.1" => {
            thread::sleep(Duration::from_secs(2));
            ("200 OK", "busybox.html")
        }
        _ => ("404 NOT FOUND", "404.html"),
    };
   
    let body = fs::read_to_string(html_path).unwrap();
    let header = format!("Content-Length: {}", body.len());
    let status = format!("HTTP/1.1 {status_code}");
    let crlf = "\r\n";
    let resp = format!("{status}{crlf}{header}{crlf}{crlf}{body}");
    stream.write_all(resp.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            pool.execute(move || {
                handle_request(stream);
            });
        }
    }
}
