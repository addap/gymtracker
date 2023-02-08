use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use web_sys::window;

use gt_core::models::AuthToken;

use crate::{api, messages::UIMessage, request_ext::RequestExt};

pub static ACTIVE_AUTH_TOKEN: Atom<Option<AuthToken>> = |_| None;

pub fn is_logged_in<'a, T>(cx: &Scope<'a, T>) -> bool {
    let auth_token = use_read(cx, ACTIVE_AUTH_TOKEN);
    auth_token.is_some()
}

pub fn init_auth_token(cx: &Scope) {
    let stored_token = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("auth_token")
        .unwrap()
        .map(AuthToken);

    // Check if auth token is still valid.
    if let Some(ref token) = stored_token {
        cx.spawn({
            let client = reqwest::Client::new();

            let request = client.post(api::AUTH_CHECK.as_str()).bearer_auth(token);
            let setter = use_set(&cx, ACTIVE_AUTH_TOKEN);
            to_owned![setter];

            async move {
                if let Ok(()) = request
                    .send()
                    .await
                    .handle_result(UIMessage::error("Reauthentication failed.".to_string()))
                    .await
                {
                    setter(stored_token);
                } else {
                    store_auth_token(None);
                }
            }
        })
    }
}

pub fn store_auth_token(opt_token: Option<AuthToken>) {
    if let Some(token) = opt_token {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("auth_token", &token)
            .unwrap();
    } else {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .delete("auth_token")
            .unwrap();
    }
}
