+++
title = "Other Blogs Highlight: Async Rust can be a pleasure to work with"
description = "An article highlight on how there are potential solutions to the difficulties with async Rust."
date = "2024-09-04"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Other Blogs"]
tags = ["rust", "async"]
+++

I've yet to find out for myself, because I don't use async Rust beyond the bounds of frameworks such as Axum. I hear there is eventual agony around the use of async Rust.

Thanks to [daily.dev](https://daily.dev), this article came to my attention:

[Async Rust can be a pleasure to work with (without `Send + Sync + 'static`)](https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static/)

I loved this particular quote:

> Some of the pain we feel when writing async Rust today comes from the fact that we're circumventing one of the core parts of Rust (using lifetimes and the drop checker for automatic cleanup) and going against the natural "grain" of the language. It's not the same as unsafe, but with 'static we are turning off a core part of the language -- and the result is painful.

Until I read this, I didn't realize just _where_ the pain comes from.

Give the piece a read, it might just open your eyes.

There are other ways and other choices. I hope to explore more as my experience grows.
