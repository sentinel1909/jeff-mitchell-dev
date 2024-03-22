---
title: "Spotlight on Actix Web"
date: "2023-11-26"
slug: "spotlight-actix-web"
category: "backend web frameworks"
tag: "actix web"
summary: "A spotlight on the backend framework Actix Web, originally written for a newsletter I started last year."
---

Welcome to the November issue of The Crusty Rustacean! It's time to reflect and opine on the world of Rust as it stands in the month of October 2023. It's very early days for this newsletter effort, and I'm still doing a lot of figuring out of what's useful and good to write about.  Thanks for your patience. Issue 5 is also quite late, which I apologize for. I guess I'm sticking to my tagline of "mostly once a month...mostly".

### In this Issue:

- What’s New in Rust?
- Crate Highlight
- What Did I Learn?
- What Did I Build?
- Learning Goals for November

### What’s New in Rust?

October brought us a new release of our beloved language. Version 1.73 went live on October 5th.  As I write this, it's  near the end of November and this is a bit of old news, but if you want to read the details regarding what 1.73 brought us, the official [Rust Blog](https://blog.rust-lang.org/2023/10/05/Rust-1.73.0.html) has you covered.

Starting this month, I'm going to take a look back at past issues of "This Week in Rust" and pick something from the last week of October to highlight. An item frequently criticized in Rust land is long compilation times. This is not something I've worried about in a giant way, as I build trivial things which at most might take a minute or two to compile. From October 21, [Kobzol’s blog](https://kobzol.github.io/rust/rustc/2023/10/21/make-rust-compiler-5percent-faster.html) provided us an article regarding a weird Linux trick which apparently can net a 5% speed boost in the compilation process. I've never been one for wierd tricks, the most I'm willing to do is change linkers as noted in Zero to Production in Rust. However, should you feel brave, give it a go!

### Crate Highlight

The crate highlight this month is [Actix Web](https://actix.rs/), a powerful, pragmatic, and extremely fast web framework for Rust. I'm familiar with it, mainly through my first attempt at working through Zero to Production in Rust. I attempted that too early in my learnings though, so put the book aside for many months. When I picked it up again in February of this year, I made the decision to do the book content with Axum instead of Actix Web. I always mean to return to Actix Web though, so this month I did.

### What Did I Learn?

*Best Practices for APIs - Enter OpenAPI*

I spent sometime reviewing the information at [OpenAPI](https://swagger.io/resources/open-api/), to begin to understand best practices around structuring APIs. I used this knowledge to get my music diary API (more in a minute) off it's feet and organized in a decent way.

*A Toe into the Actix-Web Ecosystem*

The thing I started to conquer in October was full blown middleware in Actix Web. I did this as a means of tackling traits in general. I learn by example and having a meaty, specific thing to take apart really moved me along in my understanding. I wrote a blog article called *Demystifying Middleware in Actix Web*, which I received feedback on right from Rob Ede, the lead maintainer of the Actix project. It was really neat to receive acknowledgement from a star in the Rust community.

In my journey through Actix Web, I touched on its ecosystem with the following crates:

- actix-cors
	- a crate enabling you to add Cross-origin Resource Sharing (CORS) protection to your API; implementing CORS protection helps protect your API from abuse by limiting what resources can be loaded by it  
- actix-governor
	- a crate enabling you to add rate-limiting to your API; rate-limiting helps protect against denial of service attacks

Actix Web is excellent, I like it a lot and find it easier to use than Axum. I feel that Axum leaves you on your own, which is great and beneficial, but you need to know what you're doing. Actix Web is more opinionated, but when you're learning like I am, it's useful to have those opinions which provide valuable context and best practices. I find I need both.

A side effect of diving into Actix Web was that the key details of the HTTP protocol became much less fuzzy. My programming knowledge predates networking and especially the modern internet. I've found it pretty detrimental to not understand at least the basics of how things are working under the hood.

### What Did I Build?

I built out a super simple API, using Actix Web, which enables me to record my daily music listening habits. I christened this thing "rivet-head-api". It's very much a work in progress, but the code is [here](https://github.com/sentinel1909/rivet-head-api)on GitHub if you'd like to look. It's hosted on [Shuttle](https://shuttle.rs) (of course). This API is something only for me though. It's not something I'm going to open to the general public.  Speaking of that...

One decision I made in the month of October was that my primary focus going forward is to make software that benefits me. I'm not skilled enough to make something for the world (yet?) but am perfectly fine to make something that aids me in in the day-to-day. The code I create will always be open source with an MIT license, so anyone can take it and learn from my mistakes and noobish practices.

### Learning Goals for November

I'm setting the goal of understanding the client side of the HTTP landscape as my goal for November. The app goal is to build a CLI app to go with my Rivet Head API.  cURL is all very well, but it's a bit verbose.  I'll be focusing on the reqwest crate, which is built on the foundational Hyper library.  I want to continue building my understanding of TCP/IP and HTTP in Rust.

### The Obligatory Call to Action

That's enough from me. What about you? What are you learning? What challenges are you facing learning Rust? What are your learning goals? That will wrap up the November issue! Thanks for reading! See you next month!
