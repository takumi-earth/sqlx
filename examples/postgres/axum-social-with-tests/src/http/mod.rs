use anyhow::Context;
use axum::{Extension, Router};
use sqlx::PgPool;

mod error;

mod post;
mod user;

pub use self::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(user::router())
        .merge(post::router())
        .layer(Extension(db))
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app(db).into_make_service())
        .await
        .context("failed to serve API")
}
