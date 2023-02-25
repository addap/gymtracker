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
use gt_core::models::{UserSignup, AuthToken};

pub fn RegisterPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_setter = use_set(&cx, ACTIVE_AUTH_TOKEN);
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
                    let display_message = cx.props.display_message.clone();

                    async move {
                        if username.current().is_empty()
                        || password.current().is_empty() 
                        || password2.current().is_empty() 
                        || email.current().is_empty() 
                        || display_name.current().is_empty() {
                            display_message.send(UIMessage::error("Empty input.".to_string()));
                            return;
                        }

                        if password.current() != password2.current() {
                            password.set("".to_string());
                            password2.set("".to_string());
                            display_message.send(UIMessage::error("Passwords do not match.".to_string()));
                            return;
                        }

                        let client = reqwest::Client::new();
                        let res = client.post(api::USER_REGISTER.as_str()).json(&UserSignup {
                            username: (*username.current()).clone(),
                            password: (*password.current()).clone(),
                            display_name: (*display_name.current()).clone(),
                            email: (*email.current()).clone(),
                        }).send().await
                        .handle_result::<AuthToken>(UIMessage::error("Registration failed".to_string())).await;

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
                "Register",
            }
        }
    })
}
