-- Your SQL goes here
CREATE TABLE discuss_main (
 id INTEGER NOT NULL PRIMARY KEY,
 author INTEGER NOT NULL,
 content Text NOT NULL,
 created_at Text NOT NULL,
 reply_to INTEGER NOT NULL,
 up INTEGER NOT NULL,
 down INTEGER NOT NULL,
 read_number INTEGER NOT NULL,
 sharded_number INTEGER NOT NULL,
 father_nodes INTEGER,
 son_nodes Text NOT NULL
);

CREATE TABLE main_nodes_list (
 id INTEGER NOT NULL PRIMARY KEY,
 nodes_list Text NOT NULL
);

CREATE TABLE user_list (
    id INTEGER NOT NULL PRIMARY KEY,
    key_id Text NOT NULL UNIQUE,
    show_name Text NOT NULL,
    key_status INTEGER NOT NULL
)
