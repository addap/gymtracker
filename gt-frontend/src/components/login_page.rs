#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_init_atom_root, use_read, use_set, Atom};
use log::info;
use serde_json::json;

use crate::{
    auth::{set_auth_token, ActiveAuthToken},
    API_BASE,
};
use gt_core::models::{AuthToken, UserLogin};

pub fn LoginPage(cx: Scope) -> Element {
    let setter = use_set(&cx, ActiveAuthToken);

    cx.render(rsx! {
        div {
            p { "Login page" }
            input { id: "username", name: "username", placeholder: "username" }
            input { id: "password", name: "password", r#type: "password", placeholder: "password" }
            button {
                onclick: move |_| cx.spawn({
                    let setter = setter.clone();

                    async move {
                        let client = reqwest::Client::new();
                        let res = client.post(concatcp!(API_BASE, "/user/login")).json(&UserLogin {
                            username: String::from("adrian3"),
                            password: String::from("123")
                        }).send().await;
                        if let Err(ref e) = res {
                            info!("{}", e);
                            return;
                        }
                        let token = res.unwrap().json::<AuthToken>().await.unwrap();

                        info!("{:?}", token);
                        set_auth_token(&setter, Some(token));
                        ()
                    }
                }),
                "Login",
            }
        }
    })
}
