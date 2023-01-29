mod auth;
mod components;

use auth::{init_auth_token, ActiveAuthToken};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, use_read, use_set};

use components as c;
use gt_core::models::AuthToken;
pub use gt_core::APP_BASE;

const BANNER: &'static str = "引き締めたいカラダのために！";

fn api_url(endpoint: &str) -> String {
    let base = web_sys::window().unwrap().origin();
    base + "/api" + endpoint
}

pub fn app(cx: Scope) -> Element {
    use_init_atom_root(&cx);

    let setter = use_set(&cx, ActiveAuthToken);
    init_auth_token(setter);

    cx.render(rsx! {
        Router {
            base_url: APP_BASE,
            c::Navbar {}
            p { BANNER }
            Route { to: "/login", c::LoggedOut{ c::LoginPage {} }}
            Route { to: "/register", c::LoggedOut {  c::RegisterPage {}  }}
            Route { to: "/history", c::LoggedIn { c::HistoryPage {} }}
            Route { to: "/stats", c::LoggedIn { c::StatsPage {} }}
            Route { to: "", c::MainPage {} }
        }
    })
}
