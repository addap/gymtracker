use axum::extract::{FromRequest, Json, State};
use http::{header::AUTHORIZATION, StatusCode};
use hyper::{Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use tower_http::auth::{AuthorizeRequest, RequireAuthorizationLayer};

pub mod exercise;
pub mod user;

#[derive(Clone, Copy)]
pub struct MyAuth;

impl<B> AuthorizeRequest<B> for MyAuth {
    type ResponseBody = Body;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let headers = request.headers();
        let auth = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();

        if let Some(user_id) = user::check_auth(auth) {
            // Set `user_id` as a request extension so it can be accessed by other
            // services down the stack.
            request.extensions_mut().insert(user_id);

            Ok(())
        } else {
            let unauthorized_response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap();

            Err(unauthorized_response)
        }
    }
}
