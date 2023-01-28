use dioxus::prelude::*;
use fermi::{use_atom_ref, use_read, use_set, Atom};
use std::rc::Rc;
use web_sys::{window, Storage};

use gt_core::models::AuthToken;

pub static ActiveAuthToken: Atom<Option<AuthToken>> = |_| None;

pub fn is_logged_in<'a, T>(cx: &Scope<'a, T>) -> bool {
    let auth_token = use_read(cx, ActiveAuthToken);
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
