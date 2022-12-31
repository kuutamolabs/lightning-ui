use sycamore::{prelude::*, suspense::Suspense};
use sycamore_router::navigate;

use crate::{api::Api, AppRoutes};

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            p {
                "Lightning Node Info"
            }
            Suspense(fallback=view! {cx, "Loading..." }) {
                GetInfo()
            }
        }
    }
}

#[component]
async fn GetInfo<G: Html>(cx: Scope<'_>) -> View<G> {
    let api = use_context::<Api>(cx);
    let info = api.get_info().await;
    if info.is_err() {
        navigate(&AppRoutes::Connect.to_string());
    }
    let info = info.unwrap();

    view! { cx,
        table {
            tr {
                td { "Node ID:" }
                td { (info.identity_pubkey.clone()) }
            }
            tr {
                td { "Alias:" }
                td { (info.alias) }
            }
            tr {
                td { "Pending Channels:" }
                td { (info.num_pending_channels) }
            }
            tr {
                td { "Active Channels:" }
                td { (info.num_active_channels) }
            }
            tr {
                td { "Inactive Channels:" }
                td { (info.num_inactive_channels) }
            }
            tr {
                td { "Number of Peers:" }
                td { (info.num_peers) }
            }
            tr {
                td { "Block Height:" }
                td { (info.block_height) }
            }
            tr {
                td { "Synced to chain:" }
                td { (info.synced_to_chain) }
            }
            tr {
                td { "Testnet:" }
                td { (info.testnet) }
            }
            tr {
                td { "Version:" }
                td { (info.version) }
            }
        }
    }
}
