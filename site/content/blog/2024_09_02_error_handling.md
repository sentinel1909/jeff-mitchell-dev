+++
title = "Saving the Day: Error Handling"
description = "An article providing insight on how to handle errors in Rust."
date = "2024-08-27"
authors = ["Jeff Mitchell"]
draft = true
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "errors", "error handling"]
+++

## Introduction

Today, we look at error handling in Rust. The sophistication of error handling is one of the language's biggest advantages. It's easy to gracefully handle whatever errors may arise. It can also be a source of frustration, because the compiler lets you get away with nearly nothing, expecting all paths, including error paths, to be handle. Compared to a dynamically typed language, this can feel a bit straight jacket-ish. I maintain that this rigidity is precisely what the world's software development community needs. The benefit is long-term maintainability of your software and fewer suprises when things go wrong.

Let's take a look more deeply.

## To Recover or Not to Recover...That is the Question

It's a fact of life that operations can fail, often in myriad unexpected ways. When things go wrong, what can we do? Errors come in two flavours, unrecoverable and recoverable. It's worth thinking about these classifications for a moment.

*Unrecoverable Errors*

Sometimes, things go so wrong that there's just nothing that can be done. In these instances, we halt execution of our program and provide some final message or output that will allow you the programmer to figure out what went wrong.

*Recoverable Errors*

There are many instances where, yes, something can go wrong, but whatever it was could be recovered from such that we pick ourselves up, dust off, and carry on. We can communicate some bit of information to the user such that they can fix a typo in their input or make an alternate choice that let's the program continue.

Fortunately, Rust provides several ways for us to, once again, leverage the type system and recover from errors if we choose.

## Grace, Too...The Result<T, E> Type

I wrote about the [Result](@/blog/2023/2023_04_14_resultful_thinking_error_handling_in_rust.md) but didn't do a super thorough job there.

## Closing Thoughts

## References
