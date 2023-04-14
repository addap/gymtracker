#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::use_read;
use futures_util::StreamExt;

use crate::{
    api,
    auth::ACTIVE_AUTH_TOKEN,
    components as c,
    messages::{MessageProps, UIMessage},
    request_ext::RequestExt,
    PAGE_SIZE,
};
use gt_core::models;

pub fn HistoryPage<'a>(cx: Scope<'a, MessageProps<'a>>) -> Element<'a> {
    let auth_token = use_read(&cx, ACTIVE_AUTH_TOKEN);
    let history = use_state(&cx, || Vec::<models::ExerciseSetQuery>::new());
    let search_term = use_state(&cx, || "".to_string());

    let fetch = use_coroutine(&cx, |mut rx: UnboundedReceiver<Option<u64>>| {
        to_owned![auth_token, history];
        let display_message = cx.props.display_message.clone();

        async move {
            while let Some(page_size_opt) = rx.next().await {
                let url = match page_size_opt {
                    Some(page_size) => format!("{}/{}", api::EXERCISE_SET.as_str(), page_size),
                    None => api::EXERCISE_SET.to_string(),
                };
                let client = reqwest::Client::new();
                let res = client
                    .get(url)
                    .bearer_auth(auth_token.clone().unwrap_or("".into()))
                    .send()
                    .await
                    .handle_result(UIMessage::error(
                        "Requesting exercise history failed.".to_string(),
                    ))
                    .await;

                match res {
                    Ok(h) => history.set(h),
                    Err(e) => {
                        display_message.send(e);
                    }
                }
            }
        }
    });
    use_future(&cx, (), |()| {
        to_owned![fetch];
        async move { fetch.send(Some(*PAGE_SIZE)) }
    });

    let content = {
        let filtered_history = history
            // filter by search box
            .get()
            .iter()
            .filter(|&exs| {
                let name = exs.name().to_lowercase();
                let search = search_term.current();
                name.contains(search.as_ref())
            });
        let hlist = filtered_history.map(|exs| {
            rsx! {
                c::ExerciseSet { exs: exs, display_message: cx.props.display_message }
            }
        });
        rsx! {
            div {
                class: "my-3 p-2",
                form {
                    class: "row g-1 g-sm-2",
                    div {
                        class: "form-group col-12 col-sm-auto",
                        button {
                            class: "btn btn-outline-secondary",
                            r#type: "button",
                            onclick: move |_| fetch.send(None),
                            "Show All"
                        }
                    }
                    div {
                        class: "form-group col-12 col-sm",
                        input {
                            class: "form-control",
                            value: "{ search_term }",
                            placeholder: "Search",
                            oninput: move |evt| { search_term.set(evt.value.to_lowercase()) }
                        }
                    }
                }
                if !history.current().is_empty() {
                    rsx!{
                        ul {
                            class: "list-group list-group-flush my-3",
                            hlist
                        }
                    }
                }
            }
        }
    };

    cx.render(rsx! {
        div {
            p { "History page" }
            rsx! { content }
        }
    })
}
