use super::models::{
  Db, IdObj, NewQuestion, NewTag, Question, QuestionEdit, QuestionTag, Tag, TagEdit,
};
use chrono::Local;
use futures::future::{try_join_all, TryFutureExt};
use std::fmt::Debug;
use warp::http::StatusCode;
use warp::reply;

pub async fn create_question(q: NewQuestion, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  let now = Local::now().naive_local();

  let mut tx = db.begin().await.map_err(SQLError)?;

  let id = sqlx::query!(
    "INSERT INTO questions (body, created_at, modified_at) VALUES(?,?,?)",
    q.body,
    now,
    now
  )
  .execute(&mut tx)
  .await
  .map_err(SQLError)?
  .last_insert_rowid();

  if let Some(tags) = q.tags {
    for tag in tags {
      sqlx::query!("INSERT INTO question_tags VALUES(?,?)", id, tag,)
        .execute(&mut tx)
        .await
        .map_err(SQLError)?;
    }
  }

  tx.commit().await.map_err(SQLError)?;

  Ok(reply::with_status(
    reply::json(&IdObj { id }),
    StatusCode::CREATED,
  ))
}

pub async fn list_questions(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  let questions = sqlx::query!("SELECT * from questions")
    .fetch_all(&db)
    .and_then(|qs| {
      try_join_all(qs.into_iter().map(|q| async {
        let tags = sqlx::query!(
          "SELECT tag_id from question_tags where question_id = ?",
          q.id,
        )
        .fetch_all(&db)
        .await?;

        Ok(Question {
          id: q.id,
          body: q.body,
          created_at: q.created_at,
          modified_at: q.modified_at,
          tags: tags.iter().map(|x| x.tag_id).collect(),
        })
      }))
    })
    .await
    .map_err(SQLError)?;

  Ok(warp::reply::json(&questions))
}

pub async fn list_tags(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  let tags = sqlx::query_as!(Tag, "SELECT id, name from tags")
    .fetch_all(&db)
    .await
    .map_err(SQLError)?;

  Ok(warp::reply::json(&tags))
}

pub async fn edit_question(
  json: QuestionEdit,
  db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
  let now = Local::now().naive_local();

  sqlx::query!(
    "UPDATE questions SET body = ?, modified_at = ? WHERE id = ?",
    json.body,
    now,
    json.id
  )
  .fetch_all(&db)
  .await
  .map_err(SQLError)?;

  Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_questions(id: IdObj, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  sqlx::query!("DELETE FROM questions WHERE id = ?", id.id)
    .execute(&db)
    .await
    .map_err(SQLError)?;

  Ok(StatusCode::NO_CONTENT)
}

pub async fn create_tag(tag: NewTag, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  let now = Local::now().naive_local();

  let id = sqlx::query!(
    "INSERT INTO tags (name, created_at, modified_at) VALUES(?,?,?)",
    tag.name,
    now,
    now
  )
  .execute(&db)
  .await
  .map_err(SQLError)?
  .last_insert_rowid();

  Ok(reply::with_status(
    reply::json(&IdObj { id }),
    StatusCode::CREATED,
  ))
}

pub async fn delete_tag(id: IdObj, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  sqlx::query!("DELETE FROM tags WHERE id = ?", id.id)
    .execute(&db)
    .await
    .map_err(SQLError)?;

  Ok(StatusCode::NO_CONTENT)
}

pub async fn edit_tag(json: TagEdit, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
  let now = Local::now().naive_local();

  sqlx::query!(
    "UPDATE tags SET name = ?, modified_at = ? WHERE id = ?",
    json.name,
    now,
    json.id
  )
  .fetch_all(&db)
  .await
  .map_err(SQLError)?;

  Ok(StatusCode::NO_CONTENT)
}

pub async fn add_question_tag(
  tag: QuestionTag,
  db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
  sqlx::query!(
    "INSERT INTO question_tags VALUES(?,?)",
    tag.question_id,
    tag.tag_id
  )
  .execute(&db)
  .await
  .map_err(SQLError)?;

  Ok(warp::reply())
}

pub async fn remove_question_tag(
  tag: QuestionTag,
  db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
  sqlx::query!(
    "DELETE FROM question_tags WHERE question_id = ? AND tag_id = ?",
    tag.question_id,
    tag.tag_id,
  )
  .execute(&db)
  .await
  .map_err(SQLError)?;

  Ok(warp::reply())
}

#[derive(Debug)]
struct SQLError(sqlx::Error);
impl warp::reject::Reject for SQLError {}
