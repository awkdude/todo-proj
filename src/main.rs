mod db;
mod util;
use actix_files as afs;
use actix_web as actix;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
// FIXME: Import less
use actix::{Responder, delete, get, http::header, post, put, web};
use serde_json::json;
use sqlx::{MySqlPool, Row, mysql};
use sqlx::mysql::types::MySqlTime;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

pub type IDType = i32;

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
    pub end_date: String,
    pub description: String,
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
    pub description: String,
    pub due_time: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskInfo {
    pub completion_value: i32,
}

#[derive(Debug, Clone, Serialize)]
struct PrototypeTask {
    pub proto_id: i32,
    pub title: String,
    pub frequency_type: i32,
    pub day_bits: i32,
    pub due_date: String,
    pub end_date: String,
    pub due_time: String,
    pub description: String,
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
    let err_msg = if info.username.is_empty() {
        Some("No username given")
    } else if info.fullname.is_empty() {
        Some("No name given")
    } else if info.password.is_empty() {
        Some("No password set")
    } else {
        None
    };
    if let Some(err_msg) = err_msg {
        return actix::HttpResponse::NotAcceptable()
            .content_type(header::ContentType::plaintext())
            .body(err_msg);
    }
    let sql_query = format!(
        "INSERT INTO user (username, fullname, password_hash) VALUES ('{}', '{}', {})",
        info.username,
        info.fullname,
        util::hash_string(&info.password)
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(result) => {
            let user_id: IDType = result.last_insert_id() as i32;
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

#[get("/api/progress/{user_id}/{date}")]
async fn get_progress(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let mut absolute_max = 0;
    let user_id = util::match_param::<i32>(&req, "user_id").unwrap();
    let date = util::match_param::<String>(&req, "date").unwrap();
    let date = util::parse_date(&date).unwrap();
    let (comp_sql_query, comp_max_sql_query) = if date.day == 0 {
        (
            format!(
                "SELECT CAST(COALESCE(SUM(t.completion_value), 0) AS INT) FROM task AS t JOIN recurring_task AS p ON p.proto_id = t.id WHERE (t.user_id = {} AND MONTH(t.due_date) = {} AND YEAR(t.due_date) = {})",
                user_id, date.month, date.year,
            ),
            format!(
                "SELECT CAST(COALESCE(SUM(p.completion_max), 0) AS INT) FROM task AS t JOIN recurring_task AS p ON p.proto_id = t.id WHERE (t.user_id = {} AND MONTH(t.due_date) = {} AND YEAR(t.due_date) = {})",
                user_id, date.month, date.year,
            ),
        )
    } else {
        (
            format!(
                "SELECT CAST(COALESCE(SUM(t.completion_value), 0) AS INT) FROM task AS t JOIN recurring_task AS p ON t.proto_id = p.proto_id WHERE (t.user_id = {} AND t.due_date = '{}')",
                user_id,
                date
            ),
            format!(
                "SELECT CAST(COALESCE(SUM(p.completion_max), 0) AS INT) FROM task AS t JOIN recurring_task AS p ON t.proto_id = p.proto_id WHERE (t.user_id = {} AND t.due_date = '{}')",
                user_id,
                date
            ),
        )
    };
    let value: i32 = sqlx::query_scalar(&comp_sql_query)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();
    let max_value: i32 = sqlx::query_scalar(&comp_max_sql_query)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();
    actix::HttpResponse::Ok().json(json!({"value": value, "max_value": max_value}))
}

#[get("/api/tasks/{user_id}/{date}")]
async fn get_tasks(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id = util::match_param::<i32>(&req, "user_id").unwrap();
    let date = util::match_param::<String>(&req, "date").unwrap();
    let date = util::parse_date(&date).unwrap();
    db::populate_recurring_tasks(db_pool.get_ref(), user_id, date)
        .await
        .unwrap();
    println!("DATE: {}", date);
    let sql_query = if date.day == 0 {
        format!(
            "SELECT t.id, t.proto_id AS proto_id, t.user_id, t.completion_value, t.due_date, t.due_date, t.due_time, p.proto_id, p.title, p.frequency_type, p.frequency_value, p.is_range, p.completion_max, p.description FROM task AS t JOIN recurring_task AS p ON t.proto_id = p.proto_id WHERE (p.user_id = {} AND MONTH(t.due_date) = {} AND YEAR(t.due_date) = {})",
            user_id, date.month, date.year,
        )
    } else {
        format!(
            "SELECT t.id, t.proto_id AS proto_id, t.user_id, t.completion_value, t.due_date, t.due_date, t.due_time, p.proto_id, p.title, p.frequency_type, p.frequency_value, p.is_range, p.completion_max, p.description FROM task AS t JOIN recurring_task AS p ON t.proto_id = p.proto_id WHERE (p.user_id = {} AND t.due_date = '{}') ORDER BY t.due_time",
            user_id,
            date
        )
    };
    println!("Query used: {sql_query}");
    let mut tasks: Vec<TaskInfo> = vec![];
    let result = sqlx::query(&sql_query).fetch_all(db_pool.get_ref()).await;
    // println!("GET result: {result:?}");
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
                let description = row.get::<String, &str>("description");
                let due_time = db::convert_time(row.try_get::<MySqlTime, &str>("due_time"));
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
                    description,
                    due_time,
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
    mut create_task: web::Json<CreateTaskInfo>,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id = util::match_param::<IDType>(&req, "user_id").unwrap();
    println!("{create_task:?}");
    if !create_task.is_range {
        create_task.range_max = 1;
    }
    let sql_query = format!(
        "INSERT INTO recurring_task (title, frequency_type, frequency_value, user_id, is_range, completion_max, start_date, due_time, end_date, description) VALUES ('{}', {}, {}, {}, {}, {}, DATE('{}'), {}, DATE('{}'), '{}')",
        create_task.title,
        create_task.frequency_type,
        create_task.day_bits,
        user_id,
        create_task.is_range,
        create_task.range_max,
        create_task.date,
        if !create_task.time.is_empty() {
            format!("'{}'", create_task.time.clone())
        } else {
            "NULL".to_string()
        },
        if !create_task.end_date.is_empty() {
            create_task.end_date.clone()
        } else {
            "2099-01-01".to_string()
        },
        create_task.description,
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(result) => {
            if create_task.frequency_type == 0 {
                let create_result = db::create_task(
                    db::Task {
                        proto_id: result.last_insert_id() as i32,
                        user_id,
                        due_date: create_task.date.clone(),
                        due_time: create_task.time.clone(),
                    },
                    db_pool.get_ref(),
                )
                .await;
                match create_result {
                    Ok(_) => actix::HttpResponse::Ok().finish(),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        actix::HttpResponse::NotAcceptable().finish()
                    }
                }
            } else {
                actix::HttpResponse::Ok().finish()
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
    let task_id = util::match_param::<i32>(&req, "task_id").unwrap();
    let completion_value = task.completion_value;
    let sql_query = format!(
        "UPDATE task SET completion_value = {completion_value} WHERE id = {task_id}"
    );
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(result) => {
            println!("Update for #{task_id}: {result:?}");
            actix::HttpResponse::Ok().finish()
        }
        Err(err) => {
            println!("Update error: {err:?}");
            actix::HttpResponse::NotModified().finish()
        }
    }
}

#[delete("/api/tasks/{task_id}")]
async fn delete_task(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let task_id = util::match_param::<i32>(&req, "task_id").unwrap();
    let sql_query = format!("DELETE FROM task WHERE id = {task_id}");
    println!("{sql_query}");
    match sqlx::query(&sql_query).execute(db_pool.get_ref()).await {
        Ok(result) => actix::HttpResponse::Ok().finish(),
        Err(_) => actix::HttpResponse::NotFound().finish(),
    }
}

#[delete("/api/proto_tasks/{proto_id}")]
async fn delete_proto_task(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let proto_id = util::match_param::<i32>(&req, "proto_id").unwrap();
    let sql_query = format!("DELETE FROM task WHERE proto_id = {proto_id}");
    let _ = sqlx::query(&sql_query).execute(db_pool.get_ref()).await;
    let sql_query = format!("DELETE FROM recurring_task WHERE proto_id = {proto_id}");
    let _ = sqlx::query(&sql_query).execute(db_pool.get_ref()).await;
    actix::HttpResponse::Ok().finish()
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

#[get("/rec")]
async fn rec() -> impl Responder {
    util::respond_with_html_page("static/rec.html")
}

#[get("/api/users/{user_id}")]
async fn get_users(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    use header::ContentType;
    use sqlx::Row;
    let user_id = util::match_param::<IDType>(&req, "user_id").unwrap_or(-1);
    let sql_query = format!("SELECT * FROM user WHERE id = {user_id}");
    let result = sqlx::query(&sql_query).fetch_one(db_pool.get_ref()).await;
    match result {
        Ok(row) => {
            let fullname = row.get::<&str, _>("fullname").to_string();
            let username = row.get::<&str, _>("username").to_string();
            actix::HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(UserInfo {
                    id: user_id,
                    fullname,
                    username,
                })
        }
        Err(_) => actix::HttpResponse::NotFound().finish(),
    }
}

#[get("/api/proto_tasks/{user_id}")]
async fn get_proto_tasks(
    req: actix::HttpRequest,
    db_pool: web::Data<mysql::MySqlPool>,
) -> impl Responder {
    let user_id: i32 = util::match_param(&req, "user_id").unwrap();
    let sql_query = format!("SELECT * FROM recurring_task WHERE user_id = {user_id}");
    let result = sqlx::query(&sql_query).fetch_all(db_pool.get_ref()).await;
    let mut proto_tasks: Vec<PrototypeTask> = vec![];
    match result {
        Ok(result) => {
            for row in result {
                let proto_id = row.get::<i32, &str>("proto_id");
                let title = row.get::<String, &str>("title");
                let frequency_type = row.get::<i32, &str>("frequency_type");
                let day_bits = row.get::<i32, &str>("frequency_value");
        let start_date = row.get::<chrono::NaiveDate, &str>("start_date");
        let start_date = start_date.format("%Y-%m-%d").to_string();
        let end_date = row.get::<chrono::NaiveDate, &str>("end_date");
        let end_date = end_date.format("%Y-%m-%d").to_string();
        let description = row.get::<String, &str>("description");
        let due_time = db::convert_time(row.try_get::<MySqlTime, &str>("due_time"));
                let proto_task = PrototypeTask {
                    proto_id,
                    title,
                    frequency_type,
                    day_bits,
                    due_date: start_date,
                    end_date,
                    due_time,
                    description,
                };
                proto_tasks.push(proto_task);
            }
            actix::HttpResponse::Ok().json(proto_tasks)
        }
        Err(e) => actix::HttpResponse::NotFound().finish(),
    }
}

#[actix::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("mysql://root:@loclhost:3306/planner_db".to_string());
    println!("waiting to connect");
    let db_pool = sqlx::MySqlPool::connect_lazy(database_url.as_str())
        .expect("ERROR: Failed to connect to DB");
    delete_from_args(db_pool.clone()).await;
    assert!(db::execute_sql_file("./sql/create.sql", db_pool.clone()).await);
    println!("connected");
    actix::HttpServer::new(move || {
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
            .service(delete_proto_task)
            .service(get_proto_tasks)
            .service(get_progress)
            .service(rec)
            .service(afs::Files::new("js/", "./static/js/"))
            .service(afs::Files::new("css/", "./static/css/"))
            .service(afs::Files::new("assets/", "./static/assets/"))
            .default_service(web::to(actix::HttpResponse::NotFound))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

async fn delete_from_args(db_pool: MySqlPool) {
    for arg in std::env::args().skip(1) {
        println!("{arg}");
        if arg.starts_with("delete")
            && let Some(table_name) = arg.split('=').nth(1)
        {
            if table_name == "*" {
                let _ = db::execute_sql_file("./sql/delete.sql", db_pool.clone()).await;
                break;
            } else {
                let q = format!("DROP TABLE IF EXISTS {table_name}");
                let _ = sqlx::query(&q).execute(&db_pool).await;
            }
        }
    }
}
