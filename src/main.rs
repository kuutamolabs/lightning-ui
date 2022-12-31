mod about;
mod api;
mod connect;
mod home;
mod storage;

use crate::api::Api;
use about::About;
use connect::Connect;
use home::Home;
use log::debug;
use storage::LocalStorage;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Route)]
pub enum AppRoutes {
    #[to("/lightning-ui")]
    Home,
    #[to("/lightning-ui/connect")]
    Connect,
    #[to("/lightning-ui/about")]
    About,
    #[not_found]
    NotFound,
}

impl ToString for AppRoutes {
    fn to_string(&self) -> String {
        match self {
            AppRoutes::Home => "/lightning-ui".to_string(),
            AppRoutes::Connect => "/lightning-ui/connect".to_string(),
            AppRoutes::About => "/lightning-ui/about".to_string(),
            _ => unreachable!(),
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let local_storage = LocalStorage::new();
    let api = Api::new().unwrap();
    if let Some(url) = local_storage.get_url() {
        api.set_url(url);
    }
    if let Some(macaroon) = local_storage.get_macaroon() {
        api.set_macaroon(macaroon);
    }

    provide_context(cx, api);
    provide_context(cx, local_storage);

    view! { cx, Router(
        integration=HistoryIntegration::new(),
        view=|cx, route: &ReadSignal<AppRoutes>| {
            view! { cx,
                div(class="app") {
                    (match route.get().as_ref() {
                        AppRoutes::Home => Home(cx),
                        AppRoutes::Connect => Connect(cx),
                        AppRoutes::About => About(cx),
                        AppRoutes::NotFound => view! { cx,
                            "404 Not Found"
                        },
                    })
                }
            }
        }
    ) }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    debug!("Starting Lightning GUI");

    sycamore::render(|cx| App(cx));
}
