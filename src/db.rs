use crate::util::Date;
use chrono::Datelike;
use sqlx::{MySqlPool, Row, mysql::MySqlQueryResult};

#[derive(Debug, Clone, Copy)]
enum Frequency {
    None,
    Daily,
    Weekly(i32),
}

impl Frequency {
    fn from(t: i32, value: i32) -> Self {
        match t {
            0 => Self::None,
            1 => Self::Daily,
            2 => Self::Weekly(value),
            _ => panic!("Invalid frequency!"),
        }
    }
}

pub fn day_mask_from_date(date: Date) -> i32 {
    use chrono::Weekday;
    let weekday = sqlx::types::chrono::NaiveDate::from_ymd_opt(
        date.year,
        date.month as u32,
        date.day as u32,
    )
    .unwrap()
    .weekday();

    // NOTE: chrono::Weekday order did not match mine

    1 << (match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    })
}

#[derive(Clone, Debug)]
pub struct Task {
    pub proto_id: i32,
    pub user_id: i32,
    pub due_date: String,
    pub due_time: String,
}

pub async fn create_task(
    prototype: Task,
    db_pool: &MySqlPool,
) -> sqlx::Result<MySqlQueryResult> {
    let sql_query = format!(
        "INSERT into task (proto_id, user_id, due_date, due_time) VALUES ({}, {}, DATE('{}'), {})",
        prototype.proto_id,
        prototype.user_id,
        prototype.due_date,
        if !prototype.due_time.is_empty() {
            format!("'{}'", prototype.due_time.clone())
        } else {
            "NULL".to_string()
        },
    );
    println!("{}", sql_query);

    sqlx::query(&sql_query).execute(db_pool).await
}

/* Iterates through all recurring tasks that are today or later
 * Only adds them if they eligible for given date but not yet added
*/
pub async fn populate_recurring_tasks(
    db_pool: &MySqlPool,
    user_id: i32,
    date: Date,
) -> sqlx::Result<()> {
    let date_str = date.to_string();
    let sql_query = format!(
        "SELECT * FROM recurring_task WHERE user_id = {user_id} AND start_date <= '{date_str}' AND end_date >= {date_str}"
    );

    let result = sqlx::query(&sql_query).fetch_all(db_pool).await?;

    println!("{sql_query:?}");

    for row in result.into_iter() {
        let title = row.get::<String, &str>("title");
        let frequency_type = row.get::<i32, &str>("frequency_type");
        let frequency_value = row.get::<i32, &str>("frequency_value");
        let proto_id = row.get::<i32, &str>("proto_id");
        let start_date = row.get::<chrono::NaiveDate, &str>("start_date");
        let start_date = start_date.format("%Y-%m-%d").to_string();
        let due_time = row
            .try_get::<String, &str>("due_time")
            .unwrap_or("".to_string());
        let frequency = Frequency::from(frequency_type, frequency_value);
        println!("{title}: {frequency:?} due today");

        let sql_query = match frequency {
            Frequency::Daily => Some(format!(
                "SELECT CAST(COUNT(*) AS INT) FROM task WHERE user_id = {user_id} AND proto_id = {proto_id} AND due_date = '{date_str}'"
            )),
            Frequency::Weekly(day_bits) => {
                let day_mask = day_mask_from_date(date);
                if day_bits & day_mask != 0 {
                    Some(format!(
                        "SELECT CAST(COUNT(*) AS INT) FROM task WHERE user_id = {user_id} AND proto_id = {proto_id} AND due_date = '{date_str}'"
                    ))
                } else {
                    None
                }
            }
            Frequency::None => None,
        };
        if let Some(q) = sql_query {
            println!("query for count: {q:?}");
            let count: i32 = sqlx::query_scalar(&q).fetch_one(db_pool).await.unwrap();
            println!("count for {title:?}: {count}");
            if count == 0 {
                create_task(
                    Task {
                        proto_id,
                        user_id,
                        due_date: date.to_string(),
                        due_time,
                    },
                    db_pool,
                )
                .await?;
            }
        }
    }
    Ok(())
}

pub async fn execute_sql_file(path: &str, db_pool: MySqlPool) -> bool {
    let sql_creation_script = std::fs::read_to_string(path).unwrap();
    for query in sql_creation_script.split(';') {
        let query = query.trim();
        if !query.is_empty() {
            println!("'{query}'");
            if let Err(e) = sqlx::query(query).execute(&db_pool).await {
                eprintln!("Error!: {e}");
                return false;
            }
        }
    }
    true
}
