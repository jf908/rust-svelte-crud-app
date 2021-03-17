use super::handlers;
use super::models::Db;
use warp::{http::header, http::Method, Filter};

pub fn root(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  questions_list(db.clone())
    .or(questions_create(db.clone()))
    .or(questions_delete(db.clone()))
    .or(questions_edit(db.clone()))
    .or(tag_list(db.clone()))
    .or(tag_add(db.clone()))
    .or(tag_edit(db.clone()))
    .or(tag_remove(db.clone()))
    .or(tag_create(db.clone()))
    .or(tag_delete(db.clone()))
    .with(
      warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::GET, Method::DELETE, Method::PATCH])
        .allow_headers(&[header::CONTENT_TYPE]),
    )
}

pub fn questions_list(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question")
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::list_questions)
}

pub fn questions_delete(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question")
    .and(warp::delete())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::delete_questions)
}

pub fn questions_create(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::create_question)
}

pub fn questions_edit(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question")
    .and(warp::patch())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::edit_question)
}

pub fn tag_list(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("tag")
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::list_tags)
}

pub fn tag_add(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question" / "tag")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::add_question_tag)
}

pub fn tag_remove(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("question" / "tag")
    .and(warp::delete())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::remove_question_tag)
}

pub fn tag_create(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("tag")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::create_tag)
}

pub fn tag_delete(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("tag")
    .and(warp::delete())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::delete_tag)
}

pub fn tag_edit(
  db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::path!("tag")
    .and(warp::patch())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(handlers::edit_tag)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || db.clone())
}

// #[cfg(test)]
// mod tests {
//   use crate::models::BodyObj;
//   use sqlx::SqlitePool;
//   use std::result::Result;
//   use warp::http::StatusCode;
//   use warp::test::request;

//   #[tokio::test]
//   async fn test_post() -> Result<(), anyhow::Error> {
//     let db = SqlitePool::connect("sqlite::memory:").await?;
//     let api = super::root(db);

//     let resp = request()
//       .method("POST")
//       .path("/question")
//       .json(&BodyObj {
//         body: String::from("Test Question"),
//       })
//       .reply(&api)
//       .await;

//     assert_eq!(resp.status(), StatusCode::CREATED);

//     Ok(())
//   }
// }
