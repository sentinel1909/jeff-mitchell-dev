+++
title = "This Week in Rust"
description = "A link to This Week in Rust, Issue 564"
date = "2024-09-12"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Rust Ecosystem"]
tags = ["twir"]
+++

Hi everyone, it's Thursday, which means it's time for [This Week in Rust](https://this-week-in-rust.org/blog/2024/09/11/this-week-in-rust-564/), which went live yesterday.

I've only skimmed this weeks issue. One thing that stood out to me was [Scientific Computing in Rust #2](https://scientificcomputing.rs/monthly/2024-09). I always nurse the desire to write some piece of computational fluid dynamics software using Rust, but for the moment that goal is out of reach...because skill issues. :)

## My Week in Rust

What have I been working on this week?

Per usual, I drift. I started out the week all fired up about my own static site generator in Rust. Needless to say I didn't get far. I have some frustrations with Zola (this platform), and to work through those am working on another version of this blog that will not involve a pre-made theme, but the going is slow there. I'm unfortunately impatient (see my [previous words](@/blog/2024/2024_09_01_utter_frustration_that_is_css_morning_after.md) about just wanting the answer), which often impacts my ability to truly learn. Always on my mind is building something I understand and document from the ground up, because frequently I just can't get into the minds and documentation of others.

A few weeks ago I embarked on a redo of my office intranet site using the [Yew](https://yew.rs) web framework for Rust. I discovered Yew back in 2022 and have always found it approachable and "React" like, in terms of how to structure a web site with routes, views and components. Some aspects of Yew, such as handling form input, and enabling interactivity, have troubled me and I was unable to make progress.

Until last week...

A long time ago, when Google searching about Yew and how to do things, I discovered [The Workfall Blog](https://www.workfall.com/learning/blog/). In particular, [this article](https://www.workfall.com/learning/blog/how-to-handle-forms-efficiently-in-yew-web-development/) about form handling kind of filled in some missing pieces I couldn't glean from the Yew documentation.

Over the past few days, I practiced in my own intranet site project and am pleased to say that I finally understand how to handle form input. I intend on writing up my own version of that Workfall blog article, to demonstrate my understanding and maybe help out others by creating something more up to date.

## Upcoming Articles

I remembered I started a piece about serving the web with Rocket. I'll finish that this coming weekend and get it posted. I also have a piece in the oven about error handling. I want to work a small example and give you my thoughts on Rust error handling. Aiming for early next week on both of those.

That's it for this week everyone, thanks for reading!
