//! src/lib/header.rs

use crate::components::navigation::Navigation;
use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Navigation {}
    }
}
