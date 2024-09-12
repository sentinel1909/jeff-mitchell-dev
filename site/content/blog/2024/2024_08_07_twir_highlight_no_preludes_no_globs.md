+++
title = "This Week in Rust Highlight: Don't Use Preludes and Globs"
description = "An article highlighting an interesting piece from This Week in Rust about preludes and globs..."
date = "2024-08-07"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Rust Ecosystem"]
tags = ["twir", "preludes and globs"]
+++

Happy Wednesday everyone!

I have a piece about HashMaps in the works, but am currently attempting to unravel the `entry` API, so it might be awhile yet before that piece gets published.

In the meantime, I read this [great piece](https://corrode.dev/blog/dont-use-preludes-and-globs/), currated by the "This Week in Rust" folks, regarding the use of preludes and globs.

TLDR: Don't. The author considers them an anti-pattern.

I tend to not use preludes myself. I like to pull in only what I need AND it helps me to understand specifically what I'm using from the library.

Enjoy!
