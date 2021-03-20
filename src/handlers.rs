use super::models::{
  Db, IdObj, NewQuestion, NewTag, Question, QuestionEdit, QuestionNoTags, QuestionQuery,
  QuestionTag, Tag, TagEdit,
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

pub async fn list_questions(
  query: QuestionQuery,
  db: Db,
) -> Result<impl warp::Reply, warp::Rejection> {
  let limit = query.limit.unwrap_or(100);
  let offset = query.offset.unwrap_or(0);

  let sql_query;

  let rows = if query.tags.len() <= 0 {
    sqlx::query_as::<_, QuestionNoTags>(
      "SELECT * from questions ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
  } else {
    let tag_str = query
      .tags
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(",");

    sql_query = format!(
      "SELECT * FROM questions WHERE id IN (
        SELECT question_id FROM question_tags
          WHERE question_id = id AND tag_id IN ({})
          GROUP BY question_id HAVING COUNT(*) = ?
      ) ORDER BY created_at LIMIT ? OFFSET ?",
      tag_str
    );

    sqlx::query_as::<_, QuestionNoTags>(&sql_query)
      .bind(query.tags.len() as i64)
      .bind(limit)
      .bind(offset)
  };

  let questions = rows
    .fetch_all(&db)
    .and_then(|qs| {
      try_join_all(qs.into_iter().map(|q| async {
        let tags = sqlx::query!(
          "SELECT tag_id FROM question_tags
            INNER JOIN tags ON id = tag_id
            WHERE question_id = ? 
            ORDER BY name",
          q.id
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
  let tags = sqlx::query_as!(Tag, "SELECT id, name, color FROM tags ORDER BY name")
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
    "INSERT INTO tags (name, color, created_at, modified_at) VALUES(?,?,?,?)",
    tag.name,
    tag.color,
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

  let mut tx = db.begin().await.map_err(SQLError)?;

  if let Some(name) = json.name {
    sqlx::query!(
      "UPDATE tags SET name = ?, modified_at = ? WHERE id = ?",
      name,
      now,
      json.id
    )
    .fetch_all(&mut tx)
    .await
    .map_err(SQLError)?;
  }

  if let Some(color) = json.color {
    sqlx::query!(
      "UPDATE tags SET color = ?, modified_at = ? WHERE id = ?",
      color,
      now,
      json.id
    )
    .fetch_all(&mut tx)
    .await
    .map_err(SQLError)?;
  }

  tx.commit().await.map_err(SQLError)?;

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
