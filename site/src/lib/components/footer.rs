//! src/lib/footer.rs

use chrono::{Datelike, Local};
use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    let year = Local::now().year();
    view! { cx,
        footer {
            p { "\u{00A9}" " " (year) " " "Jeffery D Mitchell All Rights Reserved" }
            a(href="https://sycamore-rs.netlify.app/") { "Made with Sycamore | " }
            a(href="https://shuttle.rs") { "Hosted on Shuttle" }
      }
    }
}
