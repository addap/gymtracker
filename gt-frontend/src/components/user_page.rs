#![allow(non_snake_case)]
use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use fermi::{use_atom_state, use_read};
use gloo_timers::future::TimeoutFuture;
use image::{imageops, io::Reader as ImageReader, ImageResult};
use log::info;
use std::io::Cursor;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    components::nav::{self, WrapperUserPicture, USER_PICTURE},
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
    to_dataurl,
};
use gt_core::models;

fn downscale_image_opt(bytes: &[u8]) -> ImageResult<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    let downscaled = imageops::resize(&img, 200, 200, imageops::FilterType::Triangle);

    let mut bytes: Vec<u8> = Vec::new();
    downscaled.write_to(
        &mut Cursor::new(&mut bytes),
        image::ImageOutputFormat::Jpeg(90),
    )?;
    Ok(bytes)
}

fn downscale_image(bytes: Vec<u8>) -> Vec<u8> {
    downscale_image_opt(&bytes).unwrap_or(bytes)
}

pub fn UserPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let display_name = use_state(&cx, || "".to_string());
    let user_picture = use_atom_state(&cx, USER_PICTURE);
    let user_picture_bytes = use_state(&cx, || Vec::new());
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

    use_future(&cx, (), |()| async move {
        crate::attachToFile();
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
                    onchange: move |_| cx.spawn({
                        to_owned![user_picture, user_picture_bytes];

                        async move {
                            loop {
                                TimeoutFuture::new(100).await;
                                if crate::getFileStringReady() {
                                    break;
                                }
                            }
                            info!("Reading base64 user image from Javascript");
                            let img_b64 = crate::getFileString();
                            let img_bytes = general_purpose::STANDARD.decode(img_b64).unwrap();

                            let downscaled_bytes = downscale_image(img_bytes);
                            let data_url = to_dataurl(&downscaled_bytes);

                            user_picture.set(WrapperUserPicture(data_url));
                            user_picture_bytes.set(downscaled_bytes);
                        }
                    })
                }
            }
            div { class: "w-100" }
            div {
                class: "col-6 col-sm-2",
                img {
                    class: "img-fluid rounded",
                    src: "{user_picture.current().0}",
                    alt: "User Picture"
                }
            }
            div { class: "w-100" }
            div {
                button {
                    r#type: "button",
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-success",
                    onclick: move |_| {
                        let user_info = models::UserInfo {
                            display_name: (*display_name.current()).clone(),
                        };
                        let bytes = (*user_picture_bytes.current()).clone();

                        cx.spawn({
                            to_owned![auth_token];
                            let display_message = cx.props.display_message.clone();

                            async move {
                                let client = reqwest::Client::new();

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

                                if !bytes.is_empty() {
                                    let res = client.post(api::USER_PICTURE.as_str())
                                        .body(bytes)
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
                            }
                        })
                    },
                    "Save"
                }
            }
            div {
                button {
                    r#type: "button",
                    class: "col-3 col-sm-1 btn btn-sm btn-outline-danger",
                    onclick: move |_| {
                        nav::reset_user_picture(&cx);
                        user_picture_bytes.set(Vec::new());

                        cx.spawn({
                            to_owned![auth_token];
                            let display_message = cx.props.display_message.clone();

                            async move {
                                let client = reqwest::Client::new();

                                let res = client.delete(api::USER_PICTURE.as_str())
                                    .bearer_auth(auth_token.unwrap_or("".into()))
                                    .send().await
                                    .handle_result(UIMessage::error("Deleting user picture failed.".to_string())).await;

                                match res {
                                    Ok(()) => {
                                        display_message.send(UIMessage::info(format!("Deleted user picture.")));
                                    }
                                    Err(e) => display_message.send(e)
                                }
                            }
                        })
                    },
                    "Delete"
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
