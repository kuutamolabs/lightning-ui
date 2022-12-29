use sycamore::prelude::*;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            p { "Lightning Network GUI by Kuutomo" }
        }
    }
}
