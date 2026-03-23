use dotenvy::dotenv;
use actix_files as afs;
use actix_web as actix;
use serde::Deserialize;
// FIXME: Import less
use actix::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, http::header, post, web,
};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

fn respond_with_html_page(path: &str) -> HttpResponse {
    let mut content = fs::read_to_string(path)
        .unwrap_or_else(|_| fs::read_to_string("static/404.html").unwrap());
    for (_macro, replacement) in HTML_MACROS {
        content = content.replace(_macro, replacement);
    }
    HttpResponse::Ok()
        .content_type(header::ContentType::html())
        .body(content)
}

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // HttpResponse::Ok().body("Hello, World")
    match query.get("mode") {
        Some(m) if m == "register" => respond_with_html_page("static/register.html"),
        _ => respond_with_html_page("static/login.html"),
    }
    // NamedFile::open_async(index_content).await
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[post("/")]
async fn login_post(web::Form(form): web::Form<LoginInfo>) -> impl Responder {
    // HttpResponse::Ok().body("Hello, World")
    //     for (name, value) in req.headers().iter() {
    // println!("{}: {}", name.as_str(), value.to_str().unwrap_or("---"));
    //     }
    println!(
        "Attempting to login as ({}, {})",
        form.username, form.password
    );
    respond_with_html_page("static/home.html")
    // NamedFile::open_async(index_content).await
}

#[get("/home")]
async fn home() -> impl Responder {
    respond_with_html_page("static/home.html")
}

// #[get("/{name}")]
// async fn _index(path: web::Path<String>) -> String {
//     format!("Welcome, {}!", path)
// }

const APP_TITLE: &str = "PRODO";

const HTML_MACROS: [(&str, &str); 1] = [("$TITLE$", APP_TITLE)];

#[actix::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    // DELETE: "mysql://newuser:1024@localhost:3306/planner_db";
    let pool = mysql::Pool::new(database_url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();
    HttpServer::new(|| {
        // .service(hello)
        App::new()
            .service(index)
            .service(login_post)
            .service(home)
            .service(afs::Files::new("js/", "./static/js/"))
            .service(afs::Files::new("css/", "./static/css/"))
            .service(afs::Files::new("assets/", "./static/assets/"))
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
