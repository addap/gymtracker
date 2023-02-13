use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use web_sys::window;

use crate::{api, messages::UIMessage, request_ext::RequestExt};
use gt_core::auth::get_claims_unverified;
use gt_core::models::AuthToken;

pub static ACTIVE_AUTH_TOKEN: Atom<Option<AuthToken>> = |_| None;

pub fn is_superuser<'a, T>(cx: &Scope<'a, T>) -> bool {
    let opt_auth_token = use_read(cx, ACTIVE_AUTH_TOKEN);
    let opt = (|| {
        let auth_token = opt_auth_token.as_ref()?;
        let claims = get_claims_unverified(auth_token).ok()?;
        let is_superuser = claims.get("adm")?;
        is_superuser.parse().ok()
    })();

    opt.unwrap_or(false)
}

pub fn is_logged_in<'a, T>(cx: &Scope<'a, T>) -> bool {
    let opt_auth_token = use_read(cx, ACTIVE_AUTH_TOKEN);
    opt_auth_token.is_some()
}

pub fn init_auth_token(cx: &Scope) {
    let setter = use_set(&cx, ACTIVE_AUTH_TOKEN);
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
