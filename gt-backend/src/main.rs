use axum::{
    body::HttpBody,
    extract::{Form, Path, Query, State},
    http::StatusCode,
    middleware,
    response::{Html, Response},
    routing::{get, get_service, post},
    Router, Server,
};
// use gt_core::{Mutation as MutationCore, Query as QueryCore};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::{env, net::SocketAddr, str::FromStr, sync::Arc};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::services::{ServeDir, ServeFile};

use gt_backend::{api, db, AppState, InnerAppState};

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

    let state = Arc::new(InnerAppState { conn });

    let unauth_api_routes = Router::new()
        .route("/user/login", post(api::user::login))
        .route("/user/register", post(api::user::register));

    let token_auth = ServiceBuilder::new().layer(middleware::from_fn_with_state(
        state.clone(),
        api::auth_middleware,
    ));
    let auth_api_routes = Router::new()
        .route(
            "/exercise/name",
            get(api::exercise::get_all_exercise_names).post(api::exercise::add_exercise_name),
        )
        .route(
            "/exercise/set",
            get(api::exercise::get_exercise_sets_for_user)
                .post(api::exercise::add_exercise_set_for_user),
        )
        .route(
            "/user/info",
            get(api::user::get_user_info).post(api::user::change_user_info),
        )
        .route(
            "/user/info_ts",
            get(api::user::get_user_info_ts).post(api::user::add_user_info_ts),
        )
        .route("/user/logout", post(api::user::logout))
        .layer(token_auth);

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
        // dioxus has a base_url property of a Router. However, I have no idea how to set it. A simple "Router { base_url: ... }" did not work.
        // to avoid the overlapping routes problem, I should set the base url
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
