use std::sync::Arc;

use sycamore::{prelude::*, suspense::Suspense};

use crate::api::Api;

#[component]
pub fn Home<G: Html>(cx: Scope, api: Arc<Api>) -> View<G> {
    view! { cx,
        div {
            p {
                "Lightning Node Info"
            }
            Suspense(fallback=view! {cx, "Loading..." }) {
                GetInfo(api)
            }
        }
    }
}

#[component]
async fn GetInfo<G: Html>(cx: Scope<'_>, api: Arc<Api>) -> View<G> {
    let info = api.get_info().await.unwrap();

    view! { cx,
        p { "Node ID: "  (info.identity_pubkey.clone()) }
    }
}
