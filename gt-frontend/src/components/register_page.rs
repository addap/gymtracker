#![allow(non_snake_case)]
use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_router::use_router;
use fermi::use_set;
use gt_core::models::{AuthToken, UserSignup};
use log::info;

use crate::{
    api_url,
    auth::{set_auth_token, ActiveAuthToken},
    APP_BASE,
};

pub fn RegisterPage(cx: Scope) -> Element {
    let auth_setter = use_set(&cx, ActiveAuthToken);
    let router = use_router(&cx);

    let username = use_state(&cx, || "".to_string());
    let password = use_state(&cx, || "".to_string());
    let password2 = use_state(&cx, || "".to_string());
    let display_name = use_state(&cx, || "".to_string());
    let email = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        div {
            p { "Register page" }
            input {
                id: "display-name",
                name: "display-name",
                placeholder: "Name",
                value: "{display_name}",
                oninput: move |evt| display_name.set(evt.value.clone())
            }
            br {}
            input {
                id: "username",
                name: "username",
                placeholder: "username",
                value: "{username}",
                oninput: move |evt| username.set(evt.value.clone())
            }
            br {}
            input {
                id: "password",
                name: "password",
                r#type: "password",
                placeholder: "password",
                value: "{password}",
                oninput: move |evt| password.set(evt.value.clone())
            }
            input {
                id: "password2",
                name: "password2",
                r#type: "password",
                placeholder: "repeat password",
                value: "{password2}",
                oninput: move |evt| password2.set(evt.value.clone())
            }
            br {}
            input {
                id: "email",
                name: "email",
                placeholder: "email",
                value: "{email}",
                oninput: move |evt| email.set(evt.value.clone())
            }
            br {}
            button {
                onclick: move |_| cx.spawn({
                    to_owned![auth_setter, router, username, password, password2, display_name, email];

                    async move {
                        let client = reqwest::Client::new();
                        let res = client.post(api_url("/user/register")).json(&UserSignup {
                            username: (*username.current()).clone(),
                            password: (*password.current()).clone(),
                            display_name: (*display_name.current()).clone(),
                            email: (*email.current()).clone(),
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
                "Register",
            }
        }
    })
}
