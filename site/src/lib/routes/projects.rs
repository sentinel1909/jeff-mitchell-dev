//! src/lib/projects/writing.rs

use mdsycx::{parse, MDSycX};
use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[component]
pub fn Projects<G: Html>(cx: Scope) -> View<G> {
    let projects_markdown = include_str!("../../content/projects.md");
    let parsed = parse::<()>(projects_markdown).unwrap();
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
