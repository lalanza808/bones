DROP TABLE posts;

CREATE TABLE posts (
   id SERIAL PRIMARY KEY,
   nick CHAR(60) NOT NULL,
   date_posted TIMESTAMP DEFAULT Now(),
   post_title TEXT NOT NULL
);

INSERT INTO posts (nick, date_posted, post_title)
VALUES ('testing', '2020-03-20 21:17:00', 'Just testing this shit out');
