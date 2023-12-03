-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id INT (10) AUTO_INCREMENT NOT NULL,
    name VARCHAR(10) NOT NULL,
    password VARCHAR(10) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS threads(
    id INT(10) AUTO_INCREMENT NOT NULL,
    title VARCHAR(30) NOT NULL,
    subject VARCHAR(20) NOT NULL,
    place VARCHAR(20) NOT NULL,
    purpose VARCHAR(50) NOT NULL,
    comment VARCHAR(100),
    url VARCHAR(10) NOT NULL,
    user_id INT(30) NOT NULL,
    user_name VARCHAR(10) NOT NULL,
    create_at VARCHAR(20) NOT NULL,
    PRIMARY KEY (id)
);