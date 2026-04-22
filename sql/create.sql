CREATE TABLE IF NOT EXISTS user (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    fullname VARCHAR(64) NOT NULL,
    password_hash BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS recurring_task (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(80) NOT NULL,
    frequency_type INT,
    frequency_value INT,
    start_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_range BOOL NOT NULL DEFAULT 0,
    completion_min INT DEFAULT 0,
    completion_max INT DEFAULT 1
);

CREATE TABLE IF NOT EXISTS task (
    id INT AUTO_INCREMENT PRIMARY KEY,
    prototype_id INT NOT NULL,
    user_id INT NOT NULL,
    completion_value INT NOT NULL DEFAULT 0,
    due_date DATE DEFAULT CURRENT_DATE,
    due_time TIME 
);
