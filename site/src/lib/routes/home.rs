//! src/lib/routes/home.rs

use mdsycx::{parse, MDSycX};
use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    let home_markdown = include_str!("../../content/home.md");
    let parsed = parse::<()>(home_markdown).unwrap();
    view! { cx,
        Header {}
        main {
            section{
                MDSycX(body=parsed.body)
            }
        }
        Footer {}
    }
}
