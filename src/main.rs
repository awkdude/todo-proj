use actix_files as afs;
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, http::header, post, web,
};
use std::collections::HashMap;
use std::fs;
use std::io;
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

#[get("/script.js")]
async fn script() -> impl Responder {
    // HttpResponse::Ok().body("Hello, World")
    afs::NamedFile::open_async("static/script.js").await
}

#[get("/themes/{theme_name}.css")]
async fn theme(path: web::Path<String>) -> impl Responder {
    // HttpResponse::Ok().body("Hello, World")
    afs::NamedFile::open_async(format!("static/themes/{}.css", path)).await
}

// #[get("/{name}")]
// async fn _index(path: web::Path<String>) -> String {
//     format!("Welcome, {}!", path)
// }

const APP_TITLE: &str = "PRODO";

const HTML_MACROS: [(&str, &str); 1] = [("$TITLE$", APP_TITLE)];

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        // .service(hello)
        App::new()
            // .route("/", web::get().to(index))
            // .route("/", web::post().to(index))
            .service(index)
            .service(script)
            .service(theme)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
