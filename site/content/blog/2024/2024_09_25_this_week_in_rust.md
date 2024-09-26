+++
title = "This Week in Rust"
description = "A link to This Week in Rust, Issue 566"
date = "2024-09-25"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Rust Ecosystem"]
tags = ["twir"]
+++

What do you know? I'm on time with [This Week in Rust](https://this-week-in-rust.org/blog/2024/09/25/this-week-in-rust-566/). For some reason I waited to see it appear in my daily.dev feed, but today I thought, don't need to do that!

[Announcing Stable V8 Bindings for Rust](https://deno.com/blog/rusty-v8-stabilized) was the story that caught my eye this week. Gonna explore this some more. I dislike JavaScript, but somehow using it from within the context of Rust intrigues me. JavaScript is unfortunately a necessary evil in today's world. I've always had difficulty learning it because it lacks the structure I'm used to from programming languages I learned in the past.

My own Rust adventure this week saw me complete the back end and front end for a Code Knowledge Base search tool for my office. Last night, with a bit of debugging help from ChatGPT, I implemented keyword search. This morning I took that code and filled in the tests and API endpoint for the companion code reference search. I made the tool live on the staging version of my office intranet site project, which is written in WebAsssembly via Yew. The build assets are deployed onto IIS where staff access the page via their browser of choice.

A bit of background about this project...

We have weekly stand up meetings on Tuesdays, and frequently staff will ask questions Building Code related questions which we as a group attempt help answer. As you can imagine, this can be come somewhat of a rabbit hole a lot of times, but it's a good way to hear opinions and perspectives on whatever the question is. I've been wanting for months (years) to develop a knowledge base of this material, which could be supplemented with external information and opinions. This week, finally, I'm proud to say that something very workable and useful has finally come together...in full stack Rust.

I've more than likely crated a raft of future me problems that I'll have to deal with at some point, but in the moment, it works and I'm proud of myself.

I'm going to push the search tool to the production site tomorrow, then finish fleshing out the backend admin panel of the API, which eases managing of new and existing information.

Overall, this project has really put me over the top with development using the Yew framework. I'm going to begin writing up a series of things about it, mostly so I have a place to refer back to. If it helps someone else, that's icing on the cake.

Fun times.
