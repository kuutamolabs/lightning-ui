use std::sync::Arc;

use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

use crate::{
    api::{Api, DEFAULT_URL},
    AppRoutes,
};

#[component]
pub fn Connect<G: Html>(cx: Scope, api: Arc<Api>) -> View<G> {
    let url = create_signal(cx, DEFAULT_URL.to_string());
    let macaroon = create_signal(cx, String::default());
    let status = create_signal(cx, String::default());

    let api = api;
    let connect = move |_| {
        api.connect(url.get().to_string(), macaroon.get().to_string());
        let api = api.clone();
        spawn_local_scoped(cx, async move {
            if let Err(e) = api.get_info().await {
                status.set(e.to_string());
            } else {
                navigate(&AppRoutes::Home.to_string());
            }
        });
    };

    view! { cx,
        div {
            p {
                "Connect to a lightning node."
            }
            table {
                tr {
                    td {
                        label { "URL" }
                    }
                    td {
                        input(type="text", bind:value=url)
                    }
                }
                tr {
                    td {
                        label { "Macaroon" }
                    }
                    td {
                        input(type="password", bind:value=macaroon)
                    }
                }
            }
            p {
                input(type="button", on:click=connect, value="Connect!")
            }
            p {
                (status.get())
            }
        }
    }
}
