use actix_files as afs;
use actix_web as actix;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
// FIXME: Import less
use actix::{Responder, delete, get, http::header, post, put, web};
use sqlx::mysql;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn respond_with_html_page(path: &str) -> impl Responder {
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

#[get("/")]
async fn index(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // HttpResponse::Ok().body("Hello, World")
    match query.get("mode") {
        Some(m) if m == "register" => respond_with_html_page("static/register.html"),
        _ => respond_with_html_page("static/login.html"),
    }
}

#[derive(Debug, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserInfo {
    pub fullname: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserInfo {
    pub fullname: String,
    pub username: String,
    pub password: String,
    // pub confirm_password: String,
}

#[post("/api/sessions")]
async fn auth_login(
     info: web::Json<LoginInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> actix::HttpResponse {
    println!(
        "Attempting to login as ({}, {})",
        info.username, info.password
    );
    let query_string = format!(
        "SELECT username, password_hash FROM users WHERE (username = '{}') AND (password_hash = {})",
        info.username,
        hash_string(&info.password)
    );
    match sqlx::query(&query_string)
        .fetch_one(db_pool.get_ref())
        .await
    {
        Ok(_) => {
            println!("Found!");
            let cookie = actix::cookie::Cookie::new("user", &info.username);
            actix::HttpResponse::Accepted().append_header(("Access-Control-Allow-Credentials", 1)).cookie(cookie).finish()
        }
        Err(e) => {
            println!("Not Found!: {e}");
            actix::HttpResponse::NotAcceptable().finish()
        }
    }
}

// #[post("/")]
// async fn login_page() -> impl Responder {
//     respond_with_html_page("static/home.html")
// }

#[get("/home")]
async fn home() -> impl Responder {
    respond_with_html_page("static/home.html")
}

#[get("/history")]
async fn history() -> impl Responder {
    respond_with_html_page("static/history.html")
}

fn hash_string(s: &str) -> u64 {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[post("/api/users")]
async fn create_user(
    info: web::Json<CreateUserInfo>,
    // web::Form(form): web::Form<CreateUserInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    println!("Attempting to create user: {:?}", info);
    let query_string = format!(
        "INSERT INTO users (username, fullname, password_hash) VALUES ('{}', '{}', {})",
        info.username,
        info.fullname,
        hash_string(&info.password)
    );
    sqlx::query(&query_string)
        .execute(db_pool.get_ref())
        .await
        .map_or_else(
            |err| {
                eprintln!("ERR: {:?}", err);
                actix::HttpResponse::NotFound().finish()
            },
            |_| actix::HttpResponse::Created().json(()),
        )
}

#[get("/api/users")]
async fn get_users(db_pool: web::Data<mysql::MySqlPool>) -> impl Responder {
    use header::ContentType;
    use sqlx::Row;
    let q = sqlx::query("SELECT * FROM users")
        .fetch_one(db_pool.get_ref())
        .await;
    q.map_or(actix::HttpResponse::NotFound().finish(), |row| {
        let fullname = row.get::<&str, _>("fullname").to_string();
        let username = row.get::<&str, _>("username").to_string();
        actix::HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(UserInfo { fullname, username })
    })
}

const APP_TITLE: &str = "PRODO";

const HTML_MACROS: [(&str, &str); 1] = [("$TITLE$", APP_TITLE)];

#[actix::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("mysql://newuser:1024@loclhost:3306/planner_db".to_string());
    println!("waiting to connect");
    let db_pool = sqlx::MySqlPool::connect_lazy(database_url.as_str())
        .expect("ERROR: Failed to connect to DB");
    let _ = dbg!(
        sqlx::query(CREATE_USER_TABLE_STATEMENT)
            .execute(&db_pool)
            .await
    );
    println!("connected");
    actix::HttpServer::new(move || {
        // .service(hello)
        actix::App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(index)
            .service(auth_login)
            // .service(login_page)
            .service(home)
            .service(history)
            .service(get_users)
            .service(create_user)
            .service(afs::Files::new("js/", "./static/js/"))
            .service(afs::Files::new("css/", "./static/css/"))
            .service(afs::Files::new("assets/", "./static/assets/"))
            // TODO: Respond with 404 page
            .default_service(web::to(actix::HttpResponse::NotFound))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

const CREATE_USER_TABLE_STATEMENT: &str = "CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    fullname VARCHAR(64) NOT NULL,
    password_hash TEXT NOT NULL, -- TODO: Make hash
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)";

const CREATE_TASK_TABLE_STATEMENT: &str = "CREATE TABLE IF NOT EXISTS tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)";
const CREATE_RECURRING_TASK_TABLE_STATEMENT: &str = "";
