//! src/lib/routes/about.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn About<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        Header {}
        article {
            h2 { "About Me" }
            p { "This will eventually have some information about me." }
        }
        Footer {}
    }
}
