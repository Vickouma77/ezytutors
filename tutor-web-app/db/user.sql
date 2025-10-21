DROP TABLE IF EXISTS ezyweb_users;

CREATE TABLE ezyweb_users (
    username VARCHAR(20) PRIMARY KEY,
    tutor_id INT,
    user_password CHAR(100) NOT NULL
);