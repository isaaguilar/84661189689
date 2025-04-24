use axum::extract::Path;
use axum::extract::State;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

struct AppState {
    db: sqlx::Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = PgPool::connect("postgresql://postgres:pass@127.0.0.1:5432/moviedb")
        .await
        .expect("No connection to database");

    let state = Arc::new(AppState { db: pool });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/movie/{id}", get(get_movie))
        .route("/movie", post(post_movie))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn get_movie(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Movie>, (StatusCode, String)> {
    // TODO
    // sqlx::query_as::<_, Movie>(
    //     r#"
    //     SELECT * FROM movies WHERE id = $1
    // "#,
    // )
    // .bind(id)
    // .fetch_one(&state.db);

    Ok(Json(Movie {
        id: String::from("()"),
        name: String::from("()"),
        year: 2000,
        was_good: false,
    }))
}

// basic handler that responds with a static string
async fn post_movie(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    match sqlx::query(
        r#"
        INSERT INTO movies 
            (id, name, year, was_good)
        VALUES 
            (
                (SELECT gen_random_uuid()),
                'Hello', 
                2000,
                false
            )
        "#,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, String::new())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert into db {e}"),
        )),
    }
}
