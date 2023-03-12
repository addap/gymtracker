#![feature(async_fn_in_trait)]

mod api;
mod auth;
mod components;
mod messages;
mod request_ext;
mod util;

use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;
use futures_util::StreamExt;
use gloo_timers::future::TimeoutFuture;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

use auth::init_auth_token;
use components as c;
pub use gt_core::APP_BASE;
use messages::UIMessage;

#[wasm_bindgen(module = "/js/rollup.js")]
extern "C" {
    static JS_BANNER: String;
    static JS_MESSAGE_TIMEOUT: i32;
    static JS_PAGE_SIZE: i32;
    static JS_LOGO: String;
}

#[wasm_bindgen]
extern "C" {
    fn attachToFile();
    fn getFileString() -> String;
    fn getFileStringReady() -> bool;
}

lazy_static! {
    static ref BANNER: String = JS_BANNER.clone();
    static ref MESSAGE_TIMEOUT: i64 = JS_MESSAGE_TIMEOUT.clone() as i64;
    static ref PAGE_SIZE: u64 = JS_PAGE_SIZE.clone() as u64;
    static ref LOGO: String = JS_LOGO.clone();
}

fn to_dataurl(bytes: &[u8]) -> String {
    let b64 = general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/jpg;base64,{}", b64);
    data_url
}

pub fn app(cx: Scope) -> Element {
    use_init_atom_root(&cx);
    init_auth_token(&cx);

    // Coroutine to display messages to the user.
    // Handler is passed to all components that need it.
    // Messages are displayed for MESSAGE_TIMEOUT milliseconds, then removed.
    let ui_messages = use_ref(&cx, || VecDeque::new());
    let cleanup_messages = use_coroutine(&cx, |mut rx: UnboundedReceiver<DateTime<Utc>>| {
        to_owned![ui_messages];

        async move {
            while let Some(delete_at) = rx.next().await {
                let now = Utc::now();
                if let Ok(wait_ms) = (delete_at - now).num_milliseconds().try_into() {
                    TimeoutFuture::new(wait_ms).await;
                }
                ui_messages.write().pop_front();
            }
        }
    });
    let display_message = use_coroutine(&cx, |mut rx: UnboundedReceiver<UIMessage>| {
        to_owned![ui_messages, cleanup_messages];

        async move {
            while let Some(ui_msg) = rx.next().await {
                let timeout = ui_msg.timeout;
                ui_messages.write().push_back(ui_msg);
                let delete_at = Utc::now() + timeout;
                cleanup_messages.send(delete_at);
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "container-fluid",
            Router {
                base_url: APP_BASE,
                c::Navbar {}
                div {
                    // In order for the messages to be positioned below the navbar we wrap everything after the navbar
                    // in a `position: relative` div.
                    // TODO messages should actually be sticky below the navbar instead of absolute.
                    style: "position: relative",
                    div {
                        style: "position: absolute; right: 0px; top: 0px; z-index: 99999",
                        c::Messages { ui_messages: ui_messages }
                    }
                    p { BANNER.clone() }
                }
                Route { to: "/login", c::LoggedOut{ c::LoginPage { display_message: display_message } }}
                Route { to: "/register", c::LoggedOut {  c::RegisterPage { display_message: display_message }  }}
                Route { to: "/admin", c::Superuser { c::AdminPage { display_message: display_message } }}
                Route { to: "/user", c::LoggedIn { c::UserPage { display_message: display_message } }}
                Route { to: "/history", c::LoggedIn { c::HistoryPage { display_message: display_message } }}
                Route { to: "/pr", c::LoggedIn { c::PRPage { display_message: display_message } }}
                Route { to: "/stats", c::LoggedIn { c::StatsPage {} }}
                Route { to: "", c::MainPage { display_message: display_message } }
            }
        }
    })
}
