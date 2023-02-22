#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use gloo_file::{Blob, ObjectUrl};
use log::info;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
};
use gt_core::models;

fn make_blob_url(bytes: &[u8]) -> ObjectUrl {
    ObjectUrl::from(Blob::new(bytes))
}

pub fn UserPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let display_name = use_state(&cx, || "".to_string());
    let user_picture_filename = use_state(&cx, || "".to_string());
    let user_picture_blob = use_state(&cx, || make_blob_url(&[]));
    let user_picture = use_state(&cx, || Vec::new());
    let body_height = use_state(&cx, || 0.0);
    let body_weight = use_state(&cx, || 0.0);
    let muscle_mass = use_state(&cx, || 0.0);
    let body_fat = use_state(&cx, || 0.0);

    let body_height_latest = use_state(&cx, || 0.0);
    let body_weight_latest = use_state(&cx, || 0.0);
    let muscle_mass_latest = use_state(&cx, || 0.0);
    let body_fat_latest = use_state(&cx, || 0.0);

    let fetch = use_future(&cx, (), |()| {
        to_owned![
            auth_token,
            display_name,
            body_height_latest,
            body_weight_latest,
            muscle_mass_latest,
            body_fat_latest
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
                    body_weight_latest.set(user_info.weight.unwrap_or(0.0));
                    body_height_latest.set(user_info.height.unwrap_or(0.0));
                    muscle_mass_latest.set(user_info.muscle_mass.unwrap_or(0.0));
                    body_fat_latest.set(user_info.body_fat.unwrap_or(0.0));
                }
                Err(e) => {
                    display_message.send(e);
                }
            }
        }
    });

    let fetch_image = use_future(&cx, (), |()| {
        to_owned![auth_token, user_picture_blob];
        //
        async move {
            let client = reqwest::Client::new();
            let res = client
                .get(api::USER_PICTURE.as_str())
                .bearer_auth(auth_token.unwrap_or("".into()))
                .send()
                .await;

            match res {
                Ok(res) => match res.bytes().await {
                    Ok(bytes) => {
                        let url = make_blob_url(bytes.as_ref());
                        // user_picture_blob.set(url);
                    }
                    Err(e) => {
                        info!("{}", e);
                    }
                },
                Err(e) => {
                    info!("{}", e);
                }
            }
        }
    });

    let user_form = rsx! {
    div {
        class: "bg-body-tertiary my-3 p-2",
        form {
            prevent_default: "onsubmit",
            enctype: "multipart/form-data",
            class: "row g-1 g-sm-2",
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
            div {
                class: "form-group col-12 col-sm-auto",
                label {
                    r#for: "user-picture",
                    "Picture"
                }
                input {
                    class: "form-control",
                    r#type: "file",
                    id: "user-picture",
                    value: "test",
                    // value: "{user_picture_filename}",
                    // placeholder: "picture",
                    onchange: move |evt| cx.spawn({
                        to_owned![user_picture, user_picture_filename, user_picture_blob];
                        // info!("{:?}", evt.files);
                        async move {
                            info!("1");
                            if let Some(formfiles) = evt.files.as_ref() {
                            info!("2");
                                if let Some(filename) = formfiles.files().get(0) {
                            info!("3");
                                    if let Some(bytes) = formfiles.read_file(filename.as_str()).await {
                                        user_picture_filename.set(filename.clone());
                                        let url = make_blob_url(&bytes);
                                        info!("{:?}", bytes);
                                        info!("{}", url.to_string());
                                        user_picture_blob.set(url);
                                        user_picture.set(bytes);
                                    }
                                }
                            }
                        }
                    }),
                }
            }
            div { class: "w-100" }
            div {
                class: "col-6 col-sm-2",
                img {
                    class: "img-fluid rounded",
                    src: "{user_picture_blob.current().to_string()}",
                    alt: "asd"
                }
            }
            div { class: "w-100" }
            div {
                button {
                    r#type: "button",
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                    onclick: move |_| cx.spawn({
                        to_owned![auth_token, display_name, user_picture];
                        let display_message = cx.props.display_message.clone();

                        async move {
                            let client = reqwest::Client::new();

                            let user_info = models::UserInfo {
                                display_name: (*display_name.current()).clone(),
                            };

                            let res = client.post(api::USER_INFO.as_str())
                                .json(&user_info)
                                .bearer_auth(auth_token.clone().unwrap_or("".into()))
                                .send().await
                                .handle_result(UIMessage::error("Submitting user info failed.".to_string())).await;

                            match res {
                                Ok(()) => {
                                    display_message.send(UIMessage::info(format!("Updated user info.")));
                                }
                                Err(e) => display_message.send(e)
                            }

                            let res = client.post(api::USER_PICTURE.as_str())
                                .body((*user_picture.current()).clone())
                                .bearer_auth(auth_token.unwrap_or("".into()))
                                .send().await
                                .handle_result(UIMessage::error("Submitting user picture failed.".to_string())).await;

                            match res {
                                Ok(()) => {
                                    display_message.send(UIMessage::info(format!("Updated user picture.")));
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
        class: "bg-body-tertiary my-3 p-2",
        form {
            class: "row g-1 g-sm-2",
            h3 {
                class: "col-12",
                "Body Info"
            }
            h5 {
                class: "col-12",
                "(set to 0 to not update)"
            }
            div {
                class: "col-12",
                p { "Current Body Info" }
                ul {
                    li { format!("Bodyweight: {} kg", body_weight_latest.current()) }
                    li { format!("Height: {} cm", body_height_latest.current()) }
                    li { format!("Muscle Mass: {} kg", muscle_mass_latest.current()) }
                    li { format!("Body Fat: {} %", body_fat_latest.current()) }
                }
            }
            div {
                class: "form-group col-12 col-sm-auto",
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
                class: "form-group col-12 col-sm-auto",
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
                class: "form-group col-12 col-sm-auto",
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
                class: "form-group col-12 col-sm-auto",
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
                    r#type: "button",
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                    onclick: move |_| cx.spawn({
                        to_owned![
                            auth_token,
                            body_height,
                            body_weight,
                            muscle_mass,
                            body_fat,
                            body_height_latest,
                            body_weight_latest,
                            muscle_mass_latest,
                            body_fat_latest
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

                                    if let Some(new_body_height) = update_if_nonzero(*body_height.current()) {
                                        body_height_latest.set(new_body_height);
                                    }
                                    if let Some(new_body_weight) = update_if_nonzero(*body_weight.current()) {
                                        body_weight_latest.set(new_body_weight);
                                    }
                                    if let Some(new_muscle_mass) = update_if_nonzero(*muscle_mass.current()) {
                                        muscle_mass_latest.set(new_muscle_mass);
                                    }
                                    if let Some(new_body_fat) = update_if_nonzero(*body_fat.current()) {
                                        body_fat_latest.set(new_body_fat);
                                    }
                                    body_height.set(0.0);
                                    body_weight.set(0.0);
                                    muscle_mass.set(0.0);
                                    body_fat.set(0.0);
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
