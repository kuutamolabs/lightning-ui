use sycamore::{futures::spawn_local_scoped, prelude::*};
use sycamore_router::navigate;

use crate::{
    api::{Api, DEFAULT_URL},
    storage::LocalStorage,
    AppRoutes,
};

#[component]
pub fn Connect<G: Html>(cx: Scope) -> View<G> {
    let storage = use_context::<LocalStorage>(cx);
    let url = storage.get_url().unwrap_or_else(|| DEFAULT_URL.to_string());
    let url = create_signal(cx, url);
    let macaroon = storage.get_macaroon().unwrap_or_default();
    let macaroon = create_signal(cx, macaroon);
    let status = create_signal(cx, String::default());

    let api = use_context::<Api>(cx);
    let connect = move |_| {
        api.set_macaroon(macaroon.get().to_string());
        api.set_url(url.get().to_string());
        storage.set_url(&url.get().to_string()).unwrap();
        storage.set_macaroon(&macaroon.get().to_string()).unwrap();

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
