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

#[derive(Debug)]
pub enum MatchParamError {
    Get,
    Parse,
}

pub fn match_param<T>(req: &actix::HttpRequest, name: &str) -> Result<T, MatchParamError>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s = req.match_info().get(name).ok_or(MatchParamError::Get)?;
    // println!("matching '{name}' => {s}");
    s.parse::<T>().map_err(|_| MatchParamError::Parse)
}

#[derive(Clone, Copy)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:02}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

pub fn parse_date(s: &str) -> Option<Date> {
    let mut iter = s.split('-');
    let year = iter.next()?.parse::<i32>().ok()?;
    let month = iter.next()?.parse::<i32>().ok()?;
    let day = iter.next()?.parse::<i32>().ok()?;
    Some(Date { year, month, day })
}

pub fn get_uri_queries(req: &actix::HttpRequest) -> HashMap<&str, &str> {
    let mut queries = HashMap::new();
    for q in req.query_string().split('&') {
        let mut iter = q.split('=');
        if let Some(name) = iter.next()
            && let Some(value) = iter.next()
        {
            queries.insert(name, value);
        }
    }
    queries
}
