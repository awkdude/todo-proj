// TODO:

const creation_queries: [&str; 3] = [
"CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    fullname VARCHAR(64) NOT NULL,
    password_hash BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);",
"CREATE TABLE IF NOT EXISTS recurring_tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(80) NOT NULL,
    frequency_type INT,
    frequency_type_value INT,
    start_date TIMESTAMP DEFAULT CURRENT TIMESTAMP,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_range BOOL NOT NULL DEFAULT 0,
    completion_min INT,
    completion_max INT
);",
"CREATE TABLE IF NOT EXISTS tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    prototype_id INT NOT NULL,
    user_id INT NOT NULL,
    completion_value INT NOT NULL DEFAULT 0
    due_date TIMESTAMP
);"
];
