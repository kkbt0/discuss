-- Your SQL goes here
CREATE TABLE discuss_main (
 id INTEGER NOT NULL PRIMARY KEY,
 author Text NOT NULL,
 content Text NOT NULL,
 created_at Text NOT NULL,
 up INTEGER NOT NULL,
 down INTEGER NOT NULL,
 read_number INTEGER NOT NULL,
 sharded_number INTEGER NOT NULL,
 father_nodes INTEGER,
 son_nodes Text NOT NULL
)
