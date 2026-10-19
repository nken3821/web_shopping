-- Your SQL goes here
create table users (
    id int primary key auto_increment,
    email varchar(100) not null,
    password_hash varchar(100) not null,
    full_name varchar(100) not null,
    user_name varchar(50) not null,
    phone varchar(10) not null,
    created_at datetime not null default current_timestamp,
    updated_at datetime not null default current_timestamp
        on update current_timestamp
)