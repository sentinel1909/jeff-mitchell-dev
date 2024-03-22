---
title: "Spotlight on Yew"
date: "2023-10-19"
slug: "spotlight-on-yew"
category: "frontend web frameworks"
tag: "yew"
summary: "A spotlight on the Yew frontend web framework, originally written for a newsletter I started last year."
---

Welcome to the October issue of The Crusty Rustacean! I don't know what happened to September and fall is now fully upon us. It's time to reflect and opine on the world of Rust as it stands in the month of September 2023. It's very early days for this newsletter effort, and I'm still doing a lot of figuring out of what's useful and good to write about. Thanks for your patience.

### In this Issue:

- What’s New in Rust?
- Crate Highlight of the Month
- What Did I Learn in September?
- What Did I Build in September?
- Learning Goals for October

### What’s New in Rust in September?

September brought us a point release for our beloved language, in the form of Rust 1.72.1. This release fixed some regressions that popped in up in Rust 1.72.0. As I write this, it's October and this is a bit of old news, but if you want to read the details the official [Rust Blog](https://blog.rust-lang.org/2023/09/19/Rust-1.72.1.html) has you covered.

The policy request for comment (RFC) that was started at the end of July by the crates.io team was moved along into the final comments phase. Again, if you're curious, the details are [here](https://blog.rust-lang.org/2023/09/22/crates-io-usage-policy-rfc.html) at the Rust Blog. You can still make comments on this RFC, should you wish to say your piece.

Lastly, if you're an Apple person (I am, I only keep a Windows computer around because family needs) then you'll want to check out [this note](https://blog.rust-lang.org/2023/09/25/Increasing-Apple-Version-Requirements.html) regarding the minimum supported Apple platform versions coming with Rust 1.74 in November. If you need to, get ready!

### Crate Highlight for the Month of September

The crate highlight for September is [Yew](https://yew.rs), which is a very React-like framework for developing frontend applications in WebAssembly. Yew was the original framework I started working with when I started to dabble with frontend using Rust. I built a very simple [Nasa Imagery Viewer](https://nasaapodviewer.shuttleapp.rs) with it. Yew was updated to [version 0.21](https://rew.rs/blog) towards the end of September. Yew has it's own concept of hooks, and version 0.21 brought an update to the signature of hooks that accept dependencies. In the past, dependences were passed as the second argument after the closure. From 0.21 onwards, dependencies are passed as the first argument. Let's see an example:

```rust

use_effect_with((), move |_| {
            let fetched_data = fetched_data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_nasa_data: NASAData = Request::get(api_url.as_ref())
                    .send()
                    .await
                    .expect("Unable to fetch data from NASA API")
                    .json()
                    .await
                    .expect("Data received from API is not valid.");
                fetched_data.set(fetched_nasa_data);
            });
            || ()
        });

```

This is the code from my NASA app that does the job of fetching the image or video from the API. You put any dependencies right up front now as the first parameter to the hook. The Yew folks explain:

> The reason behind swapping the order of dependencies in the code snippet is to address a specific use case. In situations where the same value is needed both to compute a dependency and to be moved by value into the closure, the new order simplifies the code and improves readability and ergonomics.

I like Yew and am steadily gaining confidence with it. I still have to master forms and processing of form data though. It's kind of like the last piece I need in order to just do everything with it.

Full code for NASA image project is [here](https://github.com/sentinel1909/nasa-imagery-viewr) if you would like to see it. The generated site is [hosted](https://nasaapodviewer.shuttleapp.rs) on Shuttle. Images or videos roll over every day at 9pm.

### What Did I Learn in September?

Continuing on the Yew theme, I mastered conditional rendering of HTML. My NASA image app, since it's original conception, had a lack of capability in that it couldn't render video properly. After a bit of sweating and bulging of forehead, I worked out how to do it:

```rust
// snip
html! {
        <main>
            <section>
                <h3>{ "Date: " } {&fetched_data.date}</h3>
                <h3>{ "Title: " } {&fetched_data.title}</h3>
                <h3>{ "Explanation: " } </h3>
                <p> {&fetched_data.explanation} </p>
                if &fetched_data.media_type == "image"  {
                    if let Some(hdurl) = fetched_data.hdurl.clone() {
                        <h3>{ "Image: " }</h3>
                        <img src={hdurl} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    } else {
                        <h3>{ "Image: " }</h3>
                        <img src={fetched_data.url.clone()} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
                    }
                } else {
                    <h3>{ "Video: "}</h3>
                    <iframe width="960" height="540" src={fetched_data.url.clone()}></iframe>
                }
                if let Some(copyright) = &fetched_data.copyright {
                    <h3>{ "Image by: "} {&copyright}</h3>
                } else {
                    <p>{ "Today's image or video has no attributed copyright data. Copyright may embedded in a watermark."}</p>
                }
            </section>
        </main>
    }

```

The key is the if let statement, which allows selection of what gets rendered, based on reading of the media_type data that's available within the result of the fetch from the API. I also extended the conditional rendering bit to cover the eventuality that there is no copyright attributed to the particular image or video. A different message will be displayed depending on what comes back.

It's a fun little site and I always enjoy whatever the fresh thing is on any particular day. It made me very happy to resolve this long standing issue. Long term goal is to give this site a history feature, where you can look back on a year's worth of images or videos.

### What Did I Build in September?

In addition to figuring out another piece of the Yew puzzle, I started work solidifying my skill with building APIs. I wanted something that could act as a diary of sorts to record my music listening habits. I decided to build off of Actix Web, rather than Axum, in order to try it out, see which one I preferred, and to just have another tool in the belt. The API is christened, "rivet-head", is hosted on [Shuttle](https://shuttle.rs), and works quite nicely. I haven't made a front for it. I interact by sending it new info via cURL. I do intend on making a form entry system eventually, because cURL is a bit verbose. I'm feeling much more confident with databases and SQLx, thanks to this effort. Eventually, I might try an ORM (object relational mapper) instead of raw SQLx, but for now I'd rather not give myself too many more things to learn.

[Code](https://github.com/sentinel1909/rivet-head-api) is here should you wish to beg, borrow and steal. I'll be continuing to build it out as soon as the Shuttle folks help with an error related to resource deletion, which they recently implemented.

### Learning Goals for October?

Looking back on the September goals I set, I didn't do that well at following through. I did read the authentication article over at the Shuttle blog, but didn't really practice by building anything of my own. For some reason, security, including things like authentication and sessions, has not stuck to the point where the pieces intuitively make sense. I feel like I'm at the point where I can get an API off the ground relatively quickly, which is huge. Keeping the API secure is not the last mile, as there is always something to refine and refactor, but it's a big step to making something relatively safe the world could benefit from. So, the learning goal for October will be to at least learn how to secure my rivet-head API with a key. I'd also like to figure out "full" middleware in Actix Web.

Another goal, and we'll see how it goes, is to understand how to instrument an API. I have to return to Chapter 4 of Zero to Production in Rust and give it another go. The various pieces of machinery related to tracing cause me confusion in terms of how and when to use them.

### The Obligatory Call to Action

What are you learning? What challenges are you facing learning Rust? What are your learning goals? That will wrap up the October issue! Thanks for reading! See you next month!
