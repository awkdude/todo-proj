mod db;
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
static mut DEMO_MODE: bool = false;

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

#[repr(i32)]
#[derive(Debug, Deserialize)]
pub enum Frequency {
    None,
    Daily,
    Weekly,
}

#[derive(Debug, Deserialize)]
struct CreateTaskInfo {
    pub title: String,
    pub frequency_type: i32,
    pub day_bits: i32,
    pub is_range: bool,
    pub range_min: i32,
    pub range_max: i32,
    pub date: String,
    pub time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TaskInfo {
    pub id: i32,
    pub proto_id: i32,
    pub title: String,
    pub frequency_type: i32,
    pub day_bits: i32,
    pub is_range: bool,
    pub range_min: i32,
    pub range_max: i32,
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
    let sql_query = format!(
        "SELECT id, username, password_hash FROM user WHERE (username = '{}') AND (password_hash = {})",
        info.username,
        util::hash_string(&info.password)
    );

    match sqlx::query(&sql_query).fetch_one(db_pool.get_ref()).await {
        Ok(row) => {
            println!("Found!");
            let user_id = row.get::<i32, &str>("id");
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
    let sql_query = format!(
        "INSERT INTO user (username, fullname, password_hash) VALUES ('{}', '{}', {})",
        info.username,
        info.fullname,
        util::hash_string(&info.password)
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
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

#[get("/api/progress/{user_id}")]
async fn get_progress(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let mut absolute_max = 0;
    let user_id = util::match_param::<i32>(&req, "user_id");
    let sql_query = format!("SELECT SUM(completion_max), t.id, p.id AS proto_id FROM task AS t JOIN recurring_task AS p ON t.id = p.id WHERE (t.user_id = {user_id}"); 
    let result: i32 = sqlx::query_scalar(&sql_query).fetch_one(db_pool.get_ref()).await.unwrap_or(0);
    println!("{result:?}");
    actix::HttpResponse::Ok().finish()
}

#[get("/api/tasks/{user_id}")]
async fn get_tasks(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id = util::match_param::<i32>(&req, "user_id");
    // let date_query = util::match_param::<i32>(&req, "");
    let sql_query = format!(
        "SELECT t.id, t.prototype_id AS proto_id, t.user_id, t.completion_value, t.due_date, t.due_date, t.due_time, p.id, p.title, p.frequency_type, p.frequency_value, p.is_range, p.completion_max FROM task AS t JOIN recurring_task AS p ON t.prototype_id = p.id WHERE p.user_id = {user_id}"
    );
    println!("request queries: {:?}", util::get_request_queries(&req));
    // TODO: GROUP BY t.date
    let mut tasks: Vec<TaskInfo> = vec![];
    let result = sqlx::query(&sql_query).fetch_all(db_pool.get_ref()).await;
    println!("GET result: {result:?}");
    match result {
        Ok(result) => {
            for row in result.into_iter() {
                let id = row.get::<i32, &str>("id");
                let proto_id = row.get::<i32, &str>("proto_id");
                let title = row.get::<String, &str>("title");
                let completion_value = row.get::<i32, &str>("completion_value");
                let frequency_type = row.get::<i32, &str>("frequency_type");
                let frequency_value = row.get::<i32, &str>("frequency_value");
                let user_id = row.get::<i32, &str>("user_id");
                let is_range = row.get::<bool, &str>("is_range");
                let completion_max = row.get::<i32, &str>("completion_max");
                let task = TaskInfo {
                    id,
                    proto_id,
                    title,
                    frequency_type,
                    day_bits: frequency_value,
                    is_range,
                    completion_value,
                    range_min: 0,
                    range_max: completion_max,
                };
                tasks.push(task.clone());
                println!("{task:?}");
            }
        }
        Err(err) => {}
    };
    actix::HttpResponse::Ok().json(tasks)
}

#[post("/api/tasks/{user_id}")]
async fn post_task(
    req: actix::HttpRequest,
    create_task: web::Json<CreateTaskInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id = util::match_param::<IDType>(&req, "user_id");
    println!("{create_task:?}");
    let sql_query = format!(
        "INSERT INTO recurring_task (title, frequency_type, frequency_value, user_id, is_range, completion_max) VALUES ('{}', {}, {}, {}, {}, {})",
        create_task.title,
        create_task.frequency_type,
        create_task.day_bits,
        user_id,
        create_task.is_range,
        create_task.range_max,
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(result) => {
            let sql_query = format!(
                "INSERT into task (prototype_id, user_id) VALUES ({}, {})",
                result.last_insert_id(),
                user_id
            );
            match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
                Ok(_) => actix::HttpResponse::Ok().finish(),
                Err(_) => actix::HttpResponse::NotAcceptable().finish(),
            }
        }
        Err(err) => {
            eprintln!("{:?}", err);
            actix::HttpResponse::NotAcceptable().finish()
        }
    }
}

#[put("/api/tasks/{task_id}")]
async fn update_task(
    req: actix::HttpRequest,
    task: web::Json<UpdateTaskInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    println!("attempting to update task");
    let task_id = util::match_param::<i32>(&req, "task_id");
    let completion_value = task.completion_value;
    let sql_query = format!(
        "UPDATE task SET completion_value = {completion_value} WHERE id = {task_id}"
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(_) => {
            println!("Updated task {task_id}");
            actix::HttpResponse::Ok().finish()
        }
        Err(_) => actix::HttpResponse::NotModified().finish(),
    }
}

#[delete("/api/tasks/{task_id}")]
async fn delete_task(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let task_id = util::match_param::<i32>(&req, "task_id");
    let sql_query = format!("DELETE from task WHERE id = {task_id}");
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(_) => {
            println!("Deleted task {task_id}!");
            actix::HttpResponse::Ok().finish()
        }
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
    let user_id = util::match_param::<IDType>(&req, "user_id");
    let sql_query = format!("SELECT * FROM user WHERE id = {user_id}");
    let result = sqlx::query(&sql_query).fetch_one(db_pool.get_ref()).await;
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

#[get("/api/demo")]
async fn demo_mode() -> impl Responder {
    if unsafe { DEMO_MODE } {
        actix::HttpResponse::Ok()
    } else {
        actix::HttpResponse::Forbidden()
    }
    .finish()
}

#[actix::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("mysql://root:@loclhost:3306/planner_db".to_string());
    println!("waiting to connect");
    let db_pool = sqlx::MySqlPool::connect_lazy(database_url.as_str())
        .expect("ERROR: Failed to connect to DB");
    for arg in std::env::args().skip(1) {
        if arg.contains("delete") {
            let _ = db::execute_sql_file("./sql/delete.sql", db_pool.clone()).await;
        } else if arg.contains("demo") {
            unsafe { DEMO_MODE = true };
        }
    }
    assert!(db::execute_sql_file("./sql/create.sql", db_pool.clone()).await);
    println!("connected");
    actix::HttpServer::new(move || {
        // .service(hello)
        actix::App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::redirect("/", "/login"))
            .service(auth_login)
            .service(login)
            .service(register)
            .service(home)
            .service(history)
            .service(get_users)
            .service(create_user)
            .service(post_task)
            .service(get_tasks)
            .service(update_task)
            .service(delete_task)
            .service(get_progress)
            .service(demo_mode)
            .service(afs::Files::new("js/", "./static/js/"))
            .service(afs::Files::new("css/", "./static/css/"))
            .service(afs::Files::new("assets/", "./static/assets/"))
            .default_service(web::to(actix::HttpResponse::NotFound))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
