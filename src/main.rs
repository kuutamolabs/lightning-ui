mod api;
mod connect;
mod home;

use std::sync::Arc;

use crate::api::Api;
use connect::Connect;
use home::Home;
use log::debug;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/home")]
    Home,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

#[component]
fn About<G: Html>(cx: Scope) -> View<G> {
    div().c(p().t("Lightning Network GUI by Kuutomo")).view(cx)
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let api = Arc::new(Api::new().unwrap());
    view! { cx, Router(
        integration=HistoryIntegration::new(),
        view=|cx, route: &ReadSignal<AppRoutes>| {
            view! { cx,
                div(class="app") {
                    (match route.get().as_ref() {
                        AppRoutes::Index => Connect(cx, api.clone()),
                        AppRoutes::Home => Home(cx, api.clone()),
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
