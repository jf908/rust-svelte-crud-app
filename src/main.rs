use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

mod filters;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let routes = filters::root(pool);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
