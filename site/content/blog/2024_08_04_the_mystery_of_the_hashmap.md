+++
title = "The Mystery of the Hashmap"
description = "An article highlighting the hashmap collection type in Rust"
date = "2024-08-04"
authors = ["Jeff Mitchell"]
draft = true
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "collections", "hashmap"]
+++

## Introduction

A little over two years ago, I started seriously doubling down on the Rust programming language. I found [Zero to Mastery](https://zerotomastery.io) and their Rust course. With a great head of steam, as I did the course, I also started blogging my way through the Rust Book. This head of steam continued nicely through the more basic aspects of the language, but as I entered the deeper waters, I faltered.

One of the areas I faltered in was the Hashmap collection type. I found it difficult to think up something specific to write about, but didn't want to just rehash/regurgitate the chapter content from the Rust book. So, this morning I asked ChatGPT, which is great for offering suggestions, and it gave me a few clues. Oddly enough, this interaction also gave me a clue in general for a structure for posts on this blog.

I can come up with a problem, then write about how to solve it in a structured way. In this instance, how can we use hashmaps as part of the solution to a particular problem?

Let's dive in!

## What is a HashMap?

A HashMap is a data type which stores mappings of keys to values. A hashing function is used to determine placement of these keys and values in memory. The advantage of a HashMap is that the key can be of any type. Rust considers HashMaps as a "collection" so you'll see it referred to as a collection type. It's not the most used data structure, so you have to bring it into scope in your programs.

HashMaps are stored in heap memory. Their keys must be all of the same type and their values must be all of the same type.

## The Problem

I'm terrible at keeping track of what I know. I'd like a simple way of remembering snippets of Rust code, so that I don't have to go hunting for them all the time on the internet. I'd like some tool that will be a portable memory aid. It should let me create, retrieve, update and delete items of knowledge. I'd like it to have storage so that what I enter is not lost. Finally, I want to be able to take this program with me whereever I go.

## The Solution

Admittedly, this is a little contrived, and there are probably much, much better ways to solve this problem, but we're going to leverage HashMaps and create a little CRUD (Create, Retrieve, Update, Delete) tool. It's not going to be a command line interface (CLI) tool, but will still be terminal based with a menu driven user inteface. We'll keep it modular, using functions rather than one giant blob in the main function. We'll also try to pull in many of the basic Rust data types, such as enums, and control flow mechanisms, like loop. Lastly, we're going to confine ourselves to what's available in the Rust standard library, no community crates.

### Algorithm (Design)

_Data Types - Snippet_

I've learned that in Rust, types are central. We begin by thinking about our data and the constrait that it needs to be represented by a HashMap. I want to store code snippets that I've developed. Our key could be just a word, like "Tera" for entries related to the Tera template engine. The key can just be a string. The associated value is words and characters, representing our code snippet. It could also be stored as a string. We're going to wrap our Snippet HashMap in a struct, which will let us implement the `Default` trait for it, making it easy to initialize or even outright reset our data. Our Snippet type will look like this:

```rust
#[derive(Debug)]
struct Snippet {
  item: HashMap<String, String>
}
```

We could do this without the struct, but I'm trying to practice some concepts here so bear with me. It's always a good idea to derive the `Debug` trait on any struct, so we do it for good measure.

We can then implement the `Default` trait, which will initialize and return and empty Snippet HashMap for us;

```rust
impl Default for Snippet {
  fn default() -> Self {
    item: HashMap::new()
  }
}
```

_Data Types - Menu Items_

We need something to represent the main menu of our program, which will need to present 5 choices: Create, Retrieve, Update, Delete, Exit. The natural choice here is an enum. It will look like this:

```rust
enum Menu {
  Create,
  Retrieve,
  Update,
  Delete,
  Exit
}
```

_Methods on the Snippet Type_

Once our data type is set, we can think in terms of methods on the data. What we will need will look like this:

```rust
impl Snippet {
  fn create() {
    // create method - creates new code snippets, given a key and a value
  }

  fn retrieve() {
    // retrieve method - retrieves any code snippet, given a key
    // this method will also act as a way of displaying snippets
  }

  fn update() {
    // update method - updates any code snippet, given a key
    // (Note: this one is tricky, we'll see later...)
  }

  fn delete() {
    // delete method - delete any code snippet, given a key
  }
}
```

_General Program Structure_

We'll need to declare our data types, their methods, as well as a couple of helper functions. We'll need functions which display the menu, get input from the user, and read/write to our storage medium, which will just be a file.

The program will run in a loop, presenting menu choices to the user for further action. The loop will be endless until the user chooses the "Exit" menu option, at which time the program will terminate.

We'll try to keep our main function pretty thin, it will initialize key things, then call other functions to get the job done.

### Pseudo Code

### Final Code

Here's the final code:

```rust
// src/main.rs

// dependencies

// main function
fn main() {

}
```

## References

[The Rust Programming Language: Chapter 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

## Resources

[Rust Standard Library: HashMap Collection](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

```

```
