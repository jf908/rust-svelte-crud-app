-- Add migration script here
CREATE TABLE IF NOT EXISTS questions
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  body TEXT NOT NULL,
  created_at DATETIME NOT NULL,
  modified_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS responses
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  body TEXT NOT NULL,
  created_at DATETIME NOT NULL,
  modified_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS tags
(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  created_at DATETIME NOT NULL,
  modified_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS question_tags
(
  question_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  PRIMARY KEY (question_id, tag_id),
  FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
