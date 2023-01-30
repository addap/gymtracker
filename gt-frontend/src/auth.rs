use dioxus::prelude::*;
use fermi::{use_read, Atom};
use std::rc::Rc;
use web_sys::window;

use gt_core::models::AuthToken;

pub static ACTIVE_AUTH_TOKEN: Atom<Option<AuthToken>> = |_| None;

pub fn is_logged_in<'a, T>(cx: &Scope<'a, T>) -> bool {
    let auth_token = use_read(cx, ACTIVE_AUTH_TOKEN);
    auth_token.is_some()
}

pub fn init_auth_token(setter: &Rc<dyn Fn(Option<AuthToken>)>) {
    let token = window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("auth_token")
        .unwrap()
        .map(AuthToken);

    setter(token);
}

pub fn set_auth_token(setter: &Rc<dyn Fn(Option<AuthToken>)>, opt_token: Option<AuthToken>) {
    //
    setter(opt_token.clone());

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
