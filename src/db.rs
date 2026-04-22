use sqlx::MySqlPool;

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
