//! src/lib/routes/writing.rs

use mdsycx::{parse, MDSycX};
use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn Writing<G: Html>(cx: Scope) -> View<G> {
    let writing_markdown = include_str!("../../content/writing.md");
    let parsed = parse::<()>(writing_markdown).unwrap();
    view! {cx,
        Header {}
        main {
            section{
                MDSycX(body=parsed.body)
            }
        }
        Footer {}
    }
}
