use axum::{
    body::HttpBody,
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{Html, Response},
    routing::{get, get_service, post},
    Router, Server,
};
// use gt_core::{Mutation as MutationCore, Query as QueryCore};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::{env, net::SocketAddr, str::FromStr};
use tower_cookies::CookieManagerLayer;
use tower_http::auth::RequireAuthorizationLayer;
use tower_http::services::{ServeDir, ServeFile};

use gt_backend::{api, db, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url).await?;

    // Migrate and populate database
    Migrator::up(&conn, None).await?;
    db::populate(&conn).await?;

    let state = AppState { conn };

    let unauth_api_routes = Router::new().route("/user/login", post(api::user::login));

    let auth_api_routes = Router::new()
        .route(
            "/exercise/name",
            get(api::exercise::get_all_exercise_names).post(api::exercise::add_exercise_name),
        )
        .layer(RequireAuthorizationLayer::custom(api::MyAuth));

    let app = Router::new()
        .merge(frontend_routes())
        .nest("/api", unauth_api_routes.merge(auth_api_routes))
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    println!("Serving at http://{}", &server_url);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

fn frontend_routes<Body: HttpBody + Send + 'static>() -> Router<AppState, Body> {
    let frontend_dir = env::var("FRONTEND_DIR").expect("FRONTEND_DIR is not set.");

    Router::new().nest_service(
        "/",
        get_service(
            ServeDir::new(&frontend_dir)
                .fallback(ServeFile::new(format!("{}{}", frontend_dir, "/index.html"))),
        )
        .handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    )
}

async fn handle(State(state): State<AppState>) -> &'static str {
    "hello world"
}
