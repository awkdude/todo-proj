use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut i = 0;
    for stream in listener.incoming() {
        let s = stream.unwrap();
        handle_connection(s);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty());
    let request_line = http_request.next().unwrap();
    let remaining_request: Vec<_> = http_request.collect();
    println!("Request: {remaining_request:#?}");

    if request_line == "GET / HTTP/1.1" {
        let contents = fs::read_to_string("static/index.html").unwrap();
        let length = contents.len();

        let status_line = "HTTP/1.1 200 OK";

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let status_line = "HTTP/1.1 404 NOT FOUND";

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
