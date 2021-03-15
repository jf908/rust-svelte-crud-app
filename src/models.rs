use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub type Db = sqlx::Pool<sqlx::Sqlite>;

#[derive(Serialize)]
pub struct Question {
  pub id: i64,
  pub body: String,
  pub created_at: NaiveDateTime,
  pub modified_at: NaiveDateTime,
  pub tags: Vec<i64>,
}

#[derive(Serialize)]
pub struct Tag {
  pub id: i64,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdObj {
  pub id: i64,
}

#[derive(Deserialize)]
pub struct QuestionEdit {
  pub id: i64,
  pub body: String,
}

#[derive(Deserialize)]
pub struct NewQuestion {
  pub body: String,
  pub tags: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
pub struct BodyObj {
  pub body: String,
}

#[derive(Deserialize)]
pub struct QuestionTag {
  pub question_id: i64,
  pub tag_id: i64,
}
