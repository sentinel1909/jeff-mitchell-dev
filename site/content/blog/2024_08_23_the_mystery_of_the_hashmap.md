+++
title = "The Mystery of the Hashmap"
description = "An article highlighting the hashmap collection type in Rust"
date = "2024-08-23"
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

### Final Code

Here's the final code:

```rust
// src/main.rs

// dependencies
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// type aliases
type UpdateResult = std::result::Result<(), String>;

// struct type to represent a code Snippet
#[derive(Debug, Default)]
struct Snippet {
    items: HashMap<String, String>,
}

// methods for the Snippet type
impl Snippet {
    // create method; creates a new key, value pair
    fn create(&mut self, key: String, value: String) -> Option<String> {
        self.items.insert(key, value)
    }

    // retrieve method; retrieves a given value given a key
    fn retrieve(&self, key: String) -> Option<&String> {
        self.items.get(&key)
    }

    // update method; updates the value associated with a given key
    fn update(&mut self, key: String, updated_value: String) -> UpdateResult {
        self.items
            .get_mut(&key)
            .map(|value| *value = updated_value)
            .ok_or_else(|| format!("Item '{}' not found", key))
    }

    // delete method; deletes a key, value pair given a key
    fn delete(&mut self, key: String) -> Option<String> {
        self.items.remove(&key)
    }
}

// enum type for the application menu
enum Menu {
    Create,
    Retrieve,
    Update,
    Delete,
    Exit,
}

// function to read in any saved input
fn read_data() -> std::io::Result<Vec<u8>> {
    let data = fs::read("data/items.txt")?;
    Ok(data)
}

// function to convert the saved input into our Snippet type
fn input_to_snippet(raw_data: Vec<u8>, mut snippet: Snippet) -> Snippet {
    let string_data = String::from_utf8(raw_data).unwrap();
    let key_value_pairs = string_data.split("\n").collect::<Vec<&str>>();
    let pairs: Vec<(&str, &str)> = key_value_pairs
        .iter()
        .filter_map(|pair| pair.split_once(":"))
        .collect();

    for (key, value) in &pairs {
        snippet.items.insert(key.to_string(), value.to_string());
    }
    snippet
}

// function to get user input and pass it back for use
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    input
}

// function to write output to stdout
fn write_message(message: &[u8], writer: &mut dyn Write) {
    if let Err(e) = writer.write_all(message) {
        eprintln!("Error writing to stdout: {}", e);
    }

    if let Err(e) = writer.flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

// function which triggers the appropriate program functionality, based on the user choice
fn handle_menu_choice(choice: Menu, snippet: &mut Snippet, mut handle: &mut dyn Write) {
    match choice {
        Menu::Create => {
            write_message(b"Enter the new key:", &mut handle);
            let new_key = get_user_input();
            write_message(b"Enter the new value for that key: ", &mut handle);
            let new_value = get_user_input();
            Snippet::create(snippet, new_key.clone(), new_value.clone());
            write_message(
                format!("Created new key: {} with value: {}\n", new_key, new_value).as_bytes(),
                &mut handle,
            );
        }
        Menu::Retrieve => {
            write_message(b"Enter the desired key: ", &mut handle);
            let key = get_user_input();
            match Snippet::retrieve(snippet, key) {
                Some(value) => {
                    write_message(format!("Retrieved: {:?}\n", value).as_bytes(), &mut handle)
                }
                None => {
                    write_message(b"There is no key value pair that matches\n", &mut handle);
                }
            };
        }
        Menu::Update => {
            write_message(b"Enter the desired key to update: ", &mut handle);
            let key = get_user_input();
            write_message(b"Enter the desired new value: ", &mut handle);
            let updated_value = get_user_input();
            let _result = Snippet::update(snippet, key.clone(), updated_value);
            write_message(format!("Updated {:?} successfully.\n", key).as_bytes(), &mut handle);
        }
        Menu::Delete => {
            write_message(b"Enter the desired key to delete: ", &mut handle);
            let key = get_user_input();
            let result = Snippet::delete(snippet, key);
            if let Some(deleted) = result {
                write_message(format!("Deleted: {:?}\n", deleted).as_bytes(), &mut handle);
            }

        }
        Menu::Exit => {
            write_message(b"Exiting the program.\n", &mut handle);
        }
    }
}

// main function
fn main() -> std::io::Result<()> {
    // initialize stdout for output to the terminal
    let mut handle = io::stdout();

    // initialize an instance of our snippet type
    let snippet = Snippet::default();

    // load input from the saved file
    let raw_data = read_data()?;

    // convert the input into the Snippet type
    let mut data = input_to_snippet(raw_data, snippet);

    // the main program loop; display the menu choices, act on them, exit the program if "E" is selected
    loop {
        // display the menu options
        write_message(b"Menu: \n", &mut handle);
        write_message(b"C - Create \n", &mut handle);
        write_message(b"R - Retrieve \n", &mut handle);
        write_message(b"U - Update \n", &mut handle);
        write_message(b"D - Delete \n", &mut handle);
        write_message(b"E - Exit \n", &mut handle);

        // display a message asing for the user to make a menu choice
        write_message(b"Enter your choice: \n", &mut handle);

        // trigger the appropriate menu option based on the user's choice
        match get_user_input().to_uppercase().as_str() {
            "C" => handle_menu_choice(Menu::Create, &mut data, &mut handle),
            "R" => handle_menu_choice(Menu::Retrieve, &mut data, &mut handle),
            "U" => handle_menu_choice(Menu::Update, &mut data, &mut handle),
            "D" => handle_menu_choice(Menu::Delete, &mut data, &mut handle),
            "E" => {
                handle_menu_choice(Menu::Exit, &mut data, &mut handle);
                break;
            }
            _ => {
                writeln!(handle, "Invalid Choice. Please enter C, R, U, D, or E.\n").unwrap();
            }
        };
    }

    Ok(())
}
```

## Breakdonw

Let's break the pieces down one by one.

## Code on GitHub

You can find the full code for this article here:

[memoria-rs](https://github.com/sentinel1909/memoria-rs)

## References

[The Rust Programming Language: Chapter 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

## Resources

[Rust Standard Library: HashMap Collection](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
