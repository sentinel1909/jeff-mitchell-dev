+++
title = "Yew: First Steps"
description = "An article providing insight on how to get started with the Yew web framework for Rust."
date = "2024-09-26"
authors = ["Jeff Mitchell"]
draft = true
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "web frameworks", "frontend", "yew"]
+++

# Introduction

What if I said there was a different way to build for the web?

There is, it's called [WebAssembly](https://webassembly.org/). The Rust programming language, of which this author is so fond, can target WebAssembly. This means you can leverage Rust's type system and inherent memory safety to build for the web.

WebAssembly is supported by all major browsers and is a completely viable way to build a web application. Two things are holding it back, a) the hegemony of the JavaScript libraries and frameworks, and b) the perceived difficulty of learning Rust. Frankly I'm not sure Rust front end web dev will truly ever be a thing, but for those of us that want to, it can be a happy and productive thing today.

As soon as I found out Rust could be a viable choice for front end development, I said "I'm in".

Let's take a closer look.

## Enter Yew

There are several up and coming front end frameworks that are worth looking at. The oldest, and probably most stable (don't let the 0.21 version number fool you) is [Yew](https://yew.rs). This is the one I started on and the one I've been focusing on the most. I enjoy it a lot, it's very React like and retains the familiar notion of composing a web application out of re-usable components. It has state, hooks, a router, and is pretty much ready to go for whatever you want to do.

### Setup

**Install Rust**

There is some setup to get off the ground. First thing, if you haven't already, is install Rust. The official instructions are [here](https://www.rust-lang.org/tools/install). The minimum supported Rust version for Yew is 1.76.0. Older versions will not compile. If you have Rust installed already and just need to update your toolchain, do: `rustup update`

**Install WebAssembly Target**

The compilation target for browser-based WebAssembly is called `wasm32-unknown-unknown`. Yes, I know it sounds wierd. The unknown-unknown thing bugged me for a long while, but I'm over that now. Install it:

```bash
rustup target add wasm32-unknown-unknown
```

**Install Trunk**

The easiest build tool for managing and deploying your WebAssembly creations is called [trunk](https://trunkrs.dev). It's really nice, and very straightforward to use. Install it via cargo:

```bash
cargo install --locked trunk
```

Alright, setup is done. Let's get on with a proverbial "Hello, World!", but with a Yew flavour.

### Create the Project

First thing is to navigate to whereever you save your development projects. Once there, do:

```bash
cargo new hello-yew-app
cd hello-yew-app
```

This will start a new Rust project and change you into the freshly created directory that holds it. Replace the contents of the `Cargo.toml` file with this:

```toml
[package]
name = "hello-yew-app"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = [ "csr"] }
```

This adds the latest stable version of Yew, hosted on [crates.io](https://crates.io) as a dependency to your project. It adds the "csr" feature, which enables client side rendering, meaning the application you create is actually rendered in the target browser. Yew does have server side rendering capability, but I'll get into that in a future article. For now, the end result will be a single page application, exactly the same as you'd build with React.

Now, go into `src/main.rs`, delete the boilerplate that's there and swap out with this:

```rust
// src/main.rs

// dependencies
use yew::{function_component, html};

// the root app component
#[function_component(App)]
fn app() -> Html {
  html! {
    <div>
      <h1>{ "Hello, World!" }</h1>
    </div>
  }
}

// main function, mounts the app component and renders it
fn main() {
  yew::Renderer::<App>::new().render();
}
```

The very last piece we need is an html file to target. Create `index.html` at the root of your project and add:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello Yew</title>
  </head>
  <body></body>
</html>
```

That's pretty much all that's need to get started.

## Moment of Truth

To see all the fine fruits of your labours, back at your command prompt, type:

```bash
trunk serve --open
```

This will start up trunk's hot reloading development server and open your default browser to display your handiwork. You should be greeted with a very boring "Hello, World" message in the main window of your browser. On the one hand yes, it's boring. On the other, take a step back and think about what you just did.

You built a web site...with Rust. Yes, there's HTML there, because you have to have that, but otherwise, it's all done in Rust, compiled to WebAssembly and run by your browser.

## Conclusion

This is a quick taste, to get you off the ground. The Yew documentation is a bit on the sparse side, but overall pretty good. You can dive in and learn more. In a future article, I'll get into `yew-router`, which you need to add if you want to navigate between different pages in your web site.

Thanks for reading!

## References

- [Yew Tutorial](https://yew.rs/docs/tutorial)
