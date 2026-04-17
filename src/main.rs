mod util;
use actix_files as afs;
use actix_web as actix;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
// FIXME: Import less
use actix::{Responder, delete, get, http::header, post, put, web};
use serde_json::json;
use sqlx::{Row, mysql};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

pub type IDType = u64;

// #[get("/")]
// async fn index(query: web::Query<HashMap<String, String>>) -> impl Responder {
//     // HttpResponse::Ok().body("Hello, World")
//     // match query.get("mode") {
//     //     Some(m) if m == "register" => respond_with_html_page("static/register.html"),
//     //     _ => respond_with_html_page("static/login.html"),
//     // }
//     respond_with_html_page("static/index.html")
// }

#[derive(Debug, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserInfo {
    pub id: IDType,
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

#[derive(Debug, Deserialize)]
struct CreateTaskInfo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskInfo {
    pub id: i64,
    pub title: String,
    pub completion_value: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskInfo {
    pub completion_value: i32,
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
        "SELECT id, username, password_hash FROM users WHERE (username = '{}') AND (password_hash = {})",
        info.username,
        util::hash_string(&info.password)
    );
    match sqlx::query(&query_string)
        .fetch_one(db_pool.get_ref())
        .await
    {
        Ok(row) => {
            println!("Found!");
            let user_id = row.get::<i64, &str>("id");
            actix::HttpResponse::Accepted()
                .json(json!({"user_id": user_id, "redirect": "/home"}))
        }
        Err(e) => {
            println!("Not Found!: {e}");
            actix::HttpResponse::NotAcceptable().finish()
        }
    }
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
        util::hash_string(&info.password)
    );
    match sqlx::query(&query_string).execute(db_pool.get_ref()).await {
        Ok(result) => {
            let user_id: IDType = result.last_insert_id();
            println!("Created: {:?}", result);
            actix::HttpResponse::Accepted()
                .json(json!({"user_id": user_id, "redirect": "/home"}))
        }
        Err(err) => {
            eprintln!("ERR: {:?}", err);
            actix::HttpResponse::NotFound().finish()
        }
    }
}

#[get("/tasks/{user_id}")]
async fn get_tasks(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id: IDType = req.match_info().get("user_id").unwrap().parse().unwrap();
    let query_string = format!("SELECT * FROM tasks WHERE user_id = {user_id}");
    let mut tasks: Vec<TaskInfo> = vec![];
    match sqlx::query(&query_string)
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(result) => {
            for (i, row) in result.into_iter().enumerate() {
                let id = row.get::<i64, &str>("id");
                let title = row.get::<String, &str>("title");
                let completion_value = row.get::<i32, &str>("completion_value");
                tasks.push(TaskInfo {
                    id,
                    title: title.clone(),
                    completion_value,
                });
                println!("({}, {}, {})", id, title, completion_value);
            }
        }
        Err(err) => {}
    };
    actix::HttpResponse::Ok().json(tasks)
}

#[post("/tasks/{user_id}")]
async fn post_task(
    req: actix::HttpRequest,
    create_task: web::Json<CreateTaskInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id: IDType = req.match_info().get("user_id").unwrap().parse().unwrap();
    let query_string = format!(
        "INSERT INTO tasks (title, user_id) VALUES ('{}', {user_id})",
        create_task.title
    );
    match sqlx::query(&query_string).execute(db_pool.get_ref()).await {
        Ok(_) => actix::HttpResponse::Created().finish(),
        Err(err) => {
            eprintln!("{:?}", err);
            actix::HttpResponse::NotAcceptable().finish()
        }
    }
}

#[put("/tasks/{task_id}")]
async fn update_task(
    req: actix::HttpRequest,
    task: web::Json<UpdateTaskInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    println!("attempting to update task");
    let task_id: i32 = util::match_param(&req, "task_id");
    let completion_value = task.completion_value;
    let query_string = format!(
        "UPDATE tasks SET completion_value = {completion_value} WHERE id = {task_id}"
    );
    match sqlx::query(&query_string).execute(db_pool.get_ref()).await {
        Ok(_) => {
            println!("Updated task {task_id}");
            actix::HttpResponse::Ok().finish()
        }
        Err(_) => actix::HttpResponse::NotModified().finish(),
    }
}

#[delete("/tasks/{task_id}")]
async fn delete_task(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let task_id: i32 = util::match_param(&req, "task_id");
    let query_string = format!("DELETE from tasks WHERE id = {task_id}");
    match sqlx::query(&query_string).execute(db_pool.get_ref()).await {
        Ok(_) => {
            println!("Deleted task {task_id}!");
            actix::HttpResponse::Ok().finish()
        },
        Err(_) => {
            eprintln!("Could not delete task {task_id}");
            actix::HttpResponse::NotFound().finish()
        }
    }
}

#[get("/login")]
async fn login(req: actix::HttpRequest) -> impl Responder {
    util::respond_with_html_page("static/login.html")
}

#[get("/register")]
async fn register(req: actix::HttpRequest) -> impl Responder {
    util::respond_with_html_page("static/register.html")
}

#[get("/home")]
async fn home(req: actix::HttpRequest) -> impl Responder {
    util::respond_with_html_page("static/home.html")
}

#[get("/history")]
async fn history() -> impl Responder {
    util::respond_with_html_page("static/history.html")
}

#[get("/api/users/{user_id}")]
async fn get_users(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    use header::ContentType;
    use sqlx::Row;
    let user_id: IDType = req.match_info().get("user_id").unwrap().parse().unwrap();
    let query_string = format!("SELECT * FROM users WHERE id = {user_id}");
    let result = sqlx::query(&query_string)
        .fetch_one(db_pool.get_ref())
        .await;
    result.map_or(actix::HttpResponse::NotFound().finish(), |row| {
        let fullname = row.get::<&str, _>("fullname").to_string();
        let username = row.get::<&str, _>("username").to_string();
        actix::HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(UserInfo {
                id: user_id,
                fullname,
                username,
            })
    })
}

#[actix::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("mysql://root:@loclhost:3306/planner_db".to_string());
    println!("waiting to connect");
    let db_pool = sqlx::MySqlPool::connect_lazy(database_url.as_str())
        .expect("ERROR: Failed to connect to DB");
    let _ = dbg!(
        sqlx::query_file!("./sql/create.sql")
            .execute_many(&db_pool)
            .await
    );
    println!("connected");
    actix::HttpServer::new(move || {
        // .service(hello)
        actix::App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::redirect("/", "/login"))
            .service(auth_login)
            .service(login)
            .service(register)
            // .service(login_page)
            .service(home)
            .service(history)
            .service(get_users)
            .service(create_user)
            .service(post_task)
            .service(get_tasks)
            .service(update_task)
            .service(delete_task)
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
