use std::path;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
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

    let mut content_type = "text/html";

    let mut request_line_iter = request_line.split(' ');
    let method = request_line_iter.next().unwrap();
    let request_uri = request_line_iter.next().unwrap();
    let http_version = request_line_iter.next().unwrap();

    let (content_path, content_type) = load_file(request_uri).unwrap();
    let response = if method == "GET" {
        if let Some((content_path, content_type)) = load_file(request_uri) {
        } else {
        }
    } else {
    };

    let contents = fs::read_to_string(content_path).unwrap();
    let length = contents.len();

    let status_line = "HTTP/1.1 200 OK";

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\
        Content-Type: {content_type}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    //
    // if request_line == "GET / HTTP/1.1" {
    //     let cur_dir = env::current_dir().unwrap();
    //     let contents = fs::read_to_string("static/index.html").unwrap();
    //     let length = contents.len();
    //
    //     let status_line = "HTTP/1.1 200 OK";
    //
    //     let response =
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    //   } else if request_line == "GET /script.js HTTP/1.1" {
    //     let contents = fs::read_to_string("static/script.js").unwrap();
    //     let length = contents.len();
    //     content_type = "text/javascript";
    //
    //     let status_line = "HTTP/1.1 200 OK";
    //
    //     let response =
    //         format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
    //
    //     stream.write_all(response.as_bytes()).unwrap();
    //
    //   } else {
    //
    //     let contents = fs::read_to_string("static/404.html").unwrap();
    //     let length = contents.len();
    //
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //
    //     let response =
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //
    //     stream.write_all(response.as_bytes()).unwrap();
    // }
}

enum ContentType {
    Html,
    JS,
}

fn load_file(request_uri: &str) -> Option<(String, &'static str)> {
    let output = if request_uri == "/" {
        Some(("static/index.html".to_string(), "text/html"))
    } else {
        let path = format!("static/{request_uri}");
        let content_type = match path::Path::new(path).extension() {
            Some("js") => "text/javascript",
            Some("html") => "text/html",
            _ => "?",
        };
        if fs::exists(path).unwrap_or(false) {
            Some((path, content_type))
        } else {
            None
        }
    };
    return output;
}
