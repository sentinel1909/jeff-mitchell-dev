+++
title = "String Theory"
description = "An article highlighting the two important string types in Rust"
date = "2024-07-30"
draft = false
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "collections", "strings"]
+++

## Introduction

String in Rust are hard. The difficulty stems from the different ways strings are handled by the language. This article reflects the two core types of strings and my understanding of them. It will evolve over time. Be aware, this does not offer the be-all-end-all explanation of strings. The main purpose here is to provide a jumping off point for me, and you, to dive deeper as needed. To often, I avoid writing because I feel I can't do a topic justice.

This is just an excuse.

Let's go.

## What the Heck is UTF-8?

Before we can examine the string collection type in Rust. We need to understand what is UTF-8, which stands for Unicode Transformation Format - 8 bits. I'm not going to attempt to write about it, it's well explained here: [What is UTF-8 Encoding? A Guide for Non-Programmers](https://blog.hubspot.com/website/what-is-utf-8)

In a nutshell, UTF-8 is an encoding method which covers every character in any human language, yes emojis included.

Rust encodes strings into UTF-8 by default.

# Flavours of Strings

The internet will yell at me if I don't point out the fact that there are **many** string types in Rust, I'm not going to list them all. However, in your day to day Rusting, I maintain there are really two you're going to care about:

- the `String` type
- the `&str` type

Let's get into 'em.

## Strings are Magical Paper

In preparing this article, I asked to ChatGPT to explain the String type to me like I was five. Here's what it came up with, which I thought was pretty good:

<blockquote>
Let's imagine that a String in Rust is like a magical piece of paper that can change its size to hold different words or sentences.

Magic Paper: The String is like a piece of paper that you can write on. If you write "Hello," it fits perfectly. If you want to write "Hello, world!" instead, the paper can grow to fit the new words.

Easy to Change: You can add more words to your magical paper. If you have "Hello" and you want to add " world," you can just stick it on, and the paper will get bigger to fit the new words.

Special Marker: This paper has a special marker called the end marker. It always knows where the end of your writing is, so it knows exactly how long your sentence is.

Start Small, Grow Big: You don't need to know how big your sentence will be at the beginning. You can start with a tiny piece of paper and let it grow as you add more words.

Super Flexible: You can do all sorts of things with this magical paper. You can take words away, change them, or even find out how many letters there are.

In Rust, the String type works just like this magical paper. It's a flexible, growable piece of text that you can change however you want. It's different from a &str (a string slice) which is more like a regular piece of paper that can't change size.

</blockquote>

I thought this was actually pretty good.

A `String` is a growable, mutable, owned, UTF-8 encode type. Behind any `String` is a `Vec<T>` where the type is bytes, it is a collection of bytes organized into a vector.

There are several ways to crate a `String` type, which can be confusing, but they have their time and place.

- You can create an empty string: `let mut s = String::new();`
- The `to_string()` method is available on an type that implements the `Display` trait: `let s = "I'm a string".to_string();`
- We can use `String::from()` to create a `String` from a string literal: `let s = String::from("I'm a string too!);`

The last two a functionally equivalent, your choice comes down to style and preference.

## A View to a String

That was `String`, what is a string slice? Again, let's ask ChatGPT:

<blockquote>
Let's imagine that a string slice in Rust is like a piece of paper with words on it that you can look at but not change.

Snapshot Paper: A string slice, written as &str, is like taking a picture of a part of your magical paper. This picture shows the words exactly as they are, but you can't add or remove words from the picture.

Just Looking: If you have a book and you want to show someone just one sentence from a page, you can take a picture of that sentence. They can read the sentence in the picture, but they can't change any of the words.

Point to Words: When you have a string slice, you're just pointing to some words that are already written down somewhere else. You're not making a new copy of the words; you're just saying, "Look here!"

Small Window: You can take a string slice of any part of the text. If your magical paper says "Hello, world!" you can take a slice that just shows "Hello" or "world" or even "lo, wo".

No Changing Allowed: The important thing is that with a string slice, you're not allowed to change the words. It's like looking through a window at something outside. You can see it, but you can't reach through the window to change it.

In Rust, a string slice (&str) is a way to look at a piece of text without changing it. It’s like pointing at part of a String or another piece of text and saying, "I want to see this part," but not being able to change what you see.

</blockquote>

A `&str` is a pointer into memory which holds a string slice.

## Use Cases - When to Choose `String` over `&str`

The `String` type is most useful when you don't want to worry about ownership and lifetimes. However, be aware that you're allocating memory every time you create a new `String` variable. This may incur a performance penalty that's unacceptable. Also, the `String` type doesn't implement the `Copy` trait, so using it as a field in a `struct` may cause issues and cloning is your best option...which again, may incur a performance hit that might or might not be ok.

Since a `&str` is a view to a String, you can't change it, you can only view it. The `&str` type is good for function parameters, where many times you're just passing information in and don't need to worry about ownership of the data being passed.

## Conclusion

That was your brief introduction to strings in Rust! This is a very, very watered down version, there is much more to know. Please check out the full scoop in the Rust Book.

## References

- [The Rust Programming Language, Chapter 4.3: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [The Rust Programming Language, Chapter 8.2: Storing UTF-8 Encoded Text with String](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust By Example: Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)

## Blog Articles

- [How Strings Work in Rust](https://zerotomastery.io/blog/how-strings-work-in-rust/)
