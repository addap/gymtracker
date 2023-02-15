#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
};
use gt_core::models;

pub fn UserPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let display_name = use_state(&cx, || "".to_string());
    let body_height = use_state(&cx, || 0.0);
    let body_weight = use_state(&cx, || 0.0);
    let muscle_mass = use_state(&cx, || 0.0);
    let body_fat = use_state(&cx, || 0.0);

    let fetch = use_future(&cx, (), |()| {
        to_owned![
            auth_token,
            display_name,
            body_height,
            body_weight,
            muscle_mass,
            body_fat
        ];
        let display_message = cx.props.display_message.clone();

        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api::USER_INFO.as_str())
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await
                .handle_result::<models::UserInfoQuery>(UIMessage::error(
                    "Requesting user info failed.".to_string(),
                ))
                .await;

            match res {
                Ok(user_info) => {
                    display_name.set(user_info.display_name);
                    body_weight.set(user_info.weight.unwrap_or(0.0));
                    body_height.set(user_info.height.unwrap_or(0.0));
                    muscle_mass.set(user_info.muscle_mass.unwrap_or(0.0));
                    body_fat.set(user_info.body_fat.unwrap_or(0.0));
                }
                Err(e) => {
                    display_message.send(e);
                }
            }
        }
    });

    let user_form = rsx! {
    div {
        class: "bg-body-tertiary my-2 p-2",
        div {
            class: "row gap-1",
            p {
                class: "col-12",
                "User Info"
            }
            div {
                class: "form-group col-12 col-sm-auto",
                label {
                    r#for: "display-name",
                    "Display Name"
                }
                input {
                    class: "form-control",
                    id: "display-name",
                    value: "{display_name}",
                    placeholder: "display name",
                    oninput: move |evt| display_name.set(evt.value.clone()),
                }
            }
            div { class: "w-100" }
            div {
                button {
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                    onclick: move |_| cx.spawn({
                        to_owned![auth_token, display_name];
                        let display_message = cx.props.display_message.clone();

                        async move {
                            let client = reqwest::Client::new();

                            let user_info = models::UserInfo {
                                display_name: (*display_name.current()).clone(),
                            };

                            let res = client.post(api::USER_INFO.as_str())
                                .json(&user_info).bearer_auth(auth_token.unwrap_or("".into()))
                                .send().await
                                .handle_result(UIMessage::error("Submitting user info failed.".to_string())).await;

                            match res {
                                Ok(()) => {
                                    display_message.send(UIMessage::info(format!("Updated user info.")));
                                }
                                Err(e) => display_message.send(e)
                            }
                        }
                    }),
                    "Save"
                }
            }
        }
    }};

    let user_form_ts = rsx! {
    div {
        class: "bg-body-tertiary my-2 p-2",
        div {
            class: "row gap-1",
            p {
                class: "col-12",
                "Body Info (set to 0 to not update)"
            }
            div {
                class: "form-group col-12 col-sm-2",
                label {
                    r#for: "body-weight",
                    "Bodyweight (kg)"
                }
                input {
                    class: "form-control",
                    id: "body-weight",
                    r#type: "number",
                    min: "0",
                    value: "{body_weight}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value.parse() {
                            body_weight.set(v)
                        }
                    }
                }
            }
            div {
                class: "form-group col-12 col-sm-2",
                label {
                    r#for: "body-height",
                    "Height (cm)"
                }
                input {
                    class: "form-control",
                    id: "body-height",
                    r#type: "number",
                    min: "0",
                    value: "{body_height}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value.parse() {
                            body_height.set(v)
                        }
                    }
                }
            }
            div { class: "w-100" }
            div {
                class: "form-group col-12 col-sm-2",
                label {
                    r#for: "muscle-mass",
                    "Muscle Mass (kg)"
                }
                input {
                    class: "form-control",
                    id: "muscle-mass",
                    r#type: "number",
                    min: "0",
                    value: "{muscle_mass}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value.parse() {
                            muscle_mass.set(v)
                        }
                    }
                }
            }
            div {
                class: "form-group col-12 col-sm-2",
                label {
                    r#for: "body-fat",
                    "Body Fat (%)"
                }
                input {
                    class: "form-control",
                    id: "body-fat",
                    r#type: "number",
                    min: "0",
                    value: "{body_fat}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value.parse() {
                            body_fat.set(v)
                        }
                    }
                }
            }
            div { class: "w-100" }
            div {
                button {
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                    onclick: move |_| cx.spawn({
                        to_owned![
                            auth_token,
                            body_height,
                            body_weight,
                            muscle_mass,
                            body_fat
                        ];
                        let display_message = cx.props.display_message.clone();

                        async move {
                            let client = reqwest::Client::new();
                            fn update_if_nonzero(x: f64) -> Option<f64> {
                                if x == 0.0 {
                                    None
                                } else {
                                    Some(x)
                                }
                            }

                            let user_info_ts = models::UserInfoTs {
                                height: update_if_nonzero(*body_height.current()),
                                weight: update_if_nonzero(*body_weight.current()),
                                muscle_mass: update_if_nonzero(*muscle_mass.current()),
                                body_fat: update_if_nonzero(*body_fat.current()),
                            };

                            let res = client.post(api::USER_INFO_TS.as_str())
                                .json(&user_info_ts).bearer_auth(auth_token.unwrap_or("".into()))
                                .send().await
                                .handle_result(UIMessage::error("Submitting user info failed.".to_string())).await;

                            match res {
                                Ok(()) => {
                                    display_message.send(UIMessage::info(format!("Updated user info.")));
                                }
                                Err(e) => display_message.send(e)
                            }
                        }
                    }),
                    "Save"
                }
            }
        }
    }};

    let content = match fetch.value() {
        Some(()) => {
            rsx! {
                div {
                    user_form
                    user_form_ts
                }
            }
        }
        None => {
            rsx! {
                p { "Loading" }
            }
        }
    };

    cx.render(rsx! {
        div {
            p { "User Page" }
            rsx! { content }
        }
    })
}
