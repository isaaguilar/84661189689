use axum::extract::State;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
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
        // `POST /users` goes to `create_user`
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn get_movie() -> Result<Json<Movie>, (StatusCode, String)> {
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
            "Hello", 
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
            String::from("Failed to insert into db"),
        )),
    }
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
