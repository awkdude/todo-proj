use actix::{Responder, delete, get, http::header, post, put, web};
use actix_web as actix;
use std::collections::HashMap;
use std::fs;

const APP_TITLE: &str = "Productivity Tracker";

const HTML_MACROS: [(&str, &str); 1] = [("$TITLE$", APP_TITLE)];

pub fn respond_with_html_page(path: &str) -> impl Responder {
    let mut content = fs::read_to_string(path)
        .unwrap_or_else(|_| fs::read_to_string("static/404.html").unwrap());
    for (_macro, replacement) in HTML_MACROS {
        content = content.replace(_macro, replacement);
    }
    // actix::HttpResponse::Ok()
    //     .content_type(header::ContentType::html())
    //     .body(content)
    web::Html::new(content)
}

pub fn hash_string(s: &str) -> u64 {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn match_param<T>(req: &actix::HttpRequest, name: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s = req.match_info().get(name).unwrap();
    println!("matching '{name}' => {s}");
    s.parse::<T>().unwrap()
}

pub fn get_request_queries(req: &actix::HttpRequest) -> HashMap<&str, &str> {
    let mut queries = HashMap::new();
    for q in req.query_string().split('&') {
        let mut iter = q.split('=');
        if let Some(name) = iter.next() && let Some(value) = iter.next() {
            queries.insert(name, value);
        }
    }
    queries
}
