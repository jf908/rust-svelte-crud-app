use super::handlers;
use super::models::{Db, QuestionQuery};
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
    .or(warp::fs::dir("client/build"))
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
    .and(warp::query())
    .and_then(parse_question_query)
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

async fn parse_question_query(
  params: Vec<(String, String)>,
) -> Result<QuestionQuery, warp::Rejection> {
  let mut query = QuestionQuery {
    limit: None,
    offset: None,
    tags: Vec::new(),
  };

  for (key, value) in params {
    if key == "tags" {
      query
        .tags
        .push(value.parse::<i64>().map_err(|_| InvalidQuestionQuery {})?);
    } else if key == "limit" {
      query.limit = Some(value.parse::<i64>().map_err(|_| InvalidQuestionQuery {})?);
    } else if key == "offset" {
      query.offset = Some(value.parse::<i64>().map_err(|_| InvalidQuestionQuery {})?);
    }
  }

  Ok(query)
}

#[derive(Debug)]
struct InvalidQuestionQuery;

impl warp::reject::Reject for InvalidQuestionQuery {}
