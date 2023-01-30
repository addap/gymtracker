#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::use_router;
use fermi::use_set;
use log::info;

use crate::{
    api_url,
    auth::{set_auth_token, ACTIVE_AUTH_TOKEN},
    APP_BASE,
};
use gt_core::models::{AuthToken, UserLogin};

pub fn LoginPage(cx: Scope) -> Element {
    let auth_setter = use_set(&cx, ACTIVE_AUTH_TOKEN);
    let router = use_router(&cx);

    let username = use_state(&cx, || "".to_string());
    let password = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        div {
            p { "Login page" }
            input {
                id: "username",
                name: "username",
                placeholder: "username",
                value: "{username}",
                oninput: move |evt| username.set(evt.value.clone())
            }
            input {
                id: "password",
                name: "password",
                r#type: "password",
                placeholder: "password",
                value: "{password}",
                oninput: move |evt| password.set(evt.value.clone())
            }
            button {
                onclick: move |_| cx.spawn({
                    to_owned![auth_setter, router, username, password];

                    async move {
                        let client = reqwest::Client::new();
                        let res = client.post(api_url("/user/login")).json(&UserLogin {
                            username: (*username.current()).clone(),
                            password: (*password.current()).clone(),
                        }).send().await;

                        if let Err(ref e) = res {
                            info!("{}", e);
                            return;
                        }
                        let token = res.unwrap().json::<AuthToken>().await.unwrap();

                        info!("{:?}", token);
                        set_auth_token(&auth_setter, Some(token));
                        router.navigate_to(APP_BASE);
                    }
                }),
                "Login",
            }
        }
    })
}
