//! src/app/bin.rs

use sycamore::prelude::*;

use jdm_dev_site_lib::routes::router::SiteRouter;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            SiteRouter {}
        }
    });
}
