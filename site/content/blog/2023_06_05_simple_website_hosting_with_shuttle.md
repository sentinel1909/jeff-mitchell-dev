+++
title = "Simple Website Hosting with Shuttle"
description = "A brief article on how to host static web site files on Shuttle."
date = "2023-04-14"
[taxonomies]
categories = ["How-To"]
tags = ["rust", "static files", "shuttle"]
+++

## Simple Website Hosting

I discovered [Shuttle](https://shuttle.rs) in mid-to-late 2022. I had _just_ begun to develop the slightest notion of what I was doing with the Rust, since finishing Jayson Lennon's Rust course earlier in the year on [Zero to Mastery](https://zerotomastery.io). My learning has always been through a cocktail of blog posts on Medium, random articles on the internet, and bits of Stack Overflow. Hosting my Rust projects was always a struggle for me. I had started Luca Palmieri's "Zero to Production in Rust", probably too early, and Chapter 5 of the book on deployment hadn't really stuck. I wanted to crack the deployment bits, because building things and letting them languish on my hard drive was not something I was interested in. For better for worse, I like to build in public. In reality, given the noise on the internet, it's unlikely anyone ever notices, but I enjoy the illusion that I am somehow helping others.

There was an article I hit on Medium at one point about getting your Dockerized Rust app, whatever it was, up on fly.io. I followed that article, and began to understand what I had to do. I adapted the procedure for railway.app, which I also discovered and liked a lot. I managed to get a very early "developer profile" website together, serve it up with a Rocket backend and get the whole thing served up on railway.app. It was great! I remember feeling like so much had finally clicked...it was terrific feeling.

Unfortunately, that first site was difficult to maintain, because my skills were still on the weak side, so I abandoned it.

I still longed for a way to quickly and easily put my creations out into the world.

### Enter Shuttle

In those early days of discovering the how's of deployment with Rust, the thing I came away with most was how complex it was, even for my simple little static website. There was a lot of hacking around and trial and error getting that first Dockerfile together such that railway.app would actually accept and build it. Granted, once it was done it was done, but I didn't enjoy the process of getting to a finished DockerFile. I had heard of Shuttle by this point, but when I first discovered this beautiful thing, static file hosting was not yet available. It eventually came along and somewhere early in 2023 I delightedly started to work with static files. I started contributing to the Shuttle documentation for the feature, helping flesh out the instructions and hopefully making the feature easier for others to read, understand, and implement. Shuttle neatly and tidyly takes the pain of Rust deployments away, and I wanted to do my part in showing that to the world in the clearest way I could.

### Axum, with Static Files, on Shuttle

[Axum](https://docs.rs/axum/latest/axum/index.html) is a Rust web application framework. It is relatively simple to get up and running with and it's become my go-to lately for doing things on the web with Rust. In February, I restarted Zero to Production in Rust, doing it in Axum instead of Actix. Unfortunately, I've had to halt that project because I've hit a patch that's just out of reach skill wise. I do intend on returning, but am taking a little hiatus for the time being. That hasn't stopped me from wanting to learn more about Axum.

Unlike other Rust web frameworks like Actix, Axum doesn't have a built in system of middleware. Instead, Axum relies on the Tower ecosystem for creation of support middleware. As far as hosting of static files is concerned, this used to the the territory of the axum-extra crate, which featured a router for single page applications (SPA). The static files example in the Shuttle Docs is based around use of SpaRouter to serve up the files in the static folder that you can provision by annotating your code with `#[shuttle_static_folder::StaticFolder] static_folder: PathBuf`. SpaRouter was deprecated in version 0.6.0 of axum-extra, in favour of the services available through the `tower-http` crate.

I've never liked the notion of relying on bits that are deprecated so I went on a journey, working out exactly how to replace the relative ease of SpaRouter with a similar one-liner. This [article](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/) by Robert Krahn, was an important find. Robert has kept it up to date, documenting how to get rid of SpaRouter using `ServeDir` from `tower-http::services`. The trouble is, I never understood his code. I set out with the intention of either explaining it (for all of you) or coming up with something simpler.

I've cracked it...at least for a folder of vanilla HTML/CSS/JS files saved into the static folder that Shuttle will provision for you. Let's dig in...

### ServeDir with tower-http

Let's home right in on main.rs:

```rust
// src/main.rs

use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    // build the router
    let router = Router::new()
         .nest_service("/", ServeDir::new(static_folder));

    Ok(router.into())
}
```

This code is pretty much identical to the example you've likely already seen through the Shuttle Docs. The key is `use tower_http::services::ServeDir;` We bring ServeDir into scope, which gives use a service that serves files from a given directory and all it's sub directories.

Once this is in scope, we can use it to build a router with a single endpoint at "/", by using `.nest_service`. We pass in `ServeDir::new(static_folder)`, where `static_folder` is what Shuttle has provisioned for us. Be sure to put all the files you want to host into that static folder, then, deploy to Shuttle by creating a new project container and deploying the project. All the things work, CSS is called properly when linked from the `<head>` tag and JavaScript is called when linked from the `<script>` tag. Navigation works as well within the site.

### The Shuttle Advantage

If you're a Rust afficionado, Shuttle is **the** way to build and showcase your work to the world. You can build and host your creations without twisting the fiddly dials and knobs of Docker. Leave that to the Shuttle folks. They do the hard work so we as developers don't have to.
