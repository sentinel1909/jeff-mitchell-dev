//! src/lib/navigation.rs

use sycamore::prelude::*;

#[component]
pub fn Navigation<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav {
            ul {
                li {
                    a(href="/") { "Jeff Mitchell" }
                }
                li {
                    a(href="/about") { "About Me" }
                }
                li {
                    a(href="/writing") { "Writing" }
                }
                li {
                    a(href="/projects") { "Projects" }
                }
                li {
                    a(href="https://github.com/sentinel1909") { "GitHub" }
                }
            }
        }
    }
}
