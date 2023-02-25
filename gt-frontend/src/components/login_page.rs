#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::use_router;
use fermi::use_set;

use crate::{
    api,
    auth::{store_auth_token, ACTIVE_AUTH_TOKEN},
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
    APP_BASE,
};
use gt_core::models::{AuthToken, UserLogin};

pub fn LoginPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
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
                    let display_message = cx.props.display_message.clone();

                    async move {
                        if username.current().is_empty()
                        || password.current().is_empty() {
                            display_message.send(UIMessage::error("Empty input.".to_string()));
                            return;
                        }

                        let client = reqwest::Client::new();
                        let res = client.post(api::USER_LOGIN.as_str())
                            .json(&UserLogin {
                                username: (*username.current()).clone(),
                                password: (*password.current()).clone(),
                            }).send().await
                            .handle_result::<AuthToken>(UIMessage::error("Login failed".to_string())).await;

                        match res {
                            Ok(token) => {
                                auth_setter(Some(token.clone()));
                                store_auth_token(Some(token));
                                router.navigate_to(APP_BASE);
                            }
                            Err(e) => display_message.send(e)
                        }
                    }
                }),
                "Login",
            }
        }
    })
}
