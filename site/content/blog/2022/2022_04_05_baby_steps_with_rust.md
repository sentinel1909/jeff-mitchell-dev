+++
title = "Baby Steps with Rust"
description = "Every journey starts with small steps, this is an introduction to the Rust programming language."
date = "2022-04-05"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "hello world"]
+++

### Why?

Late in 2020, I decided I needed to properly learn to code. The goal is to modernize myself and build things that will help myself and eventually the world around me. I’ve danced with several things but at the beginning of 2022 decided to double down on the Rust language. Rust is an immensely powerful tool which can be used to create pretty much any kind of software imaginable. It has a steep learning curve, but it’s well worth mastering. Rust is inherently safe and performant, has excellent tools, and some of the best compiler help I’ve ever experienced.

Rust is available for Windows, Linux, and macOS and generally can be used to create software for any/all of these platforms. I say generally because you will hit the odd occasion where certain libraries (referred to as “crates” in the Rust world) are only compiled for say Linux but not Windows or macOS. I hit this last year when following a tutorial to build a vim clone, one of the supporting crates was only compiled for Linux. The project wouldn’t work on Windows.

This is a short article which will help you get off the ground should you wish to explore this language.

### Assumptions:

- a solid foundation of general computer skills
- comfort with the command line (on the platform of your choice)
- experience in another compiled programming language (I don’t recommend coming to Rust cold, you should pick something else first if you have no previous experience)
- you’re using Visual Studio Code as your editor

### Concepts:

Rust is a compiled language, meaning the programs you write need to be translated (compiled) into a format that your computer can use. This is different from an interpreted language, in which you can write your programs and have the computer immediately do the work so you can see the result. If you’d like a more in-depth explanation of the difference, this article on freeCodeCamp is a great start.

Rust uses a system of packages, crates, and modules in which to organize code, The Rust Book states:

> Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system

Cargo is Rust’s package management tool. Cargo allows you to build, test, run and deploy your program. You can create Rust programs without it, but I don’t recommend that. I will use Cargo exclusively for the rest of this article as well as in everything I write/do going forward.

### Installation:

If you haven’t already, follow the official installation instructions for your computer platform to get Rust installed.

Greeting

It’s a cliche, but let’s make a program that says hello and displays your name. Create a directory in a location of your choosing and type:

```bash
cargo new HelloName
```

This will create a new binary project in a directory called HelloWorld. Rust programs fall into two broad types, binaries and libraries. A binary is an independent program which can be run on its own. A library cannot be run on its own and is meant to be used as a dependency internally to your program or by other Rust programs.

Next, type:

```bash
cd HelloName
```

This will change you into the freshly created HelloName project directory. From here, fire up Visual Studio code by typing:

```bash
code .
```

Visual Studio Code will launch and open the HelloWorld directory and display all its contents in the file browser. You should see a directory called src, click on it to reveal main.rs, the entry point of the bare bones Rust program that Cargo created for you. Cargo actually mocks up all the code we need without anything additional. The code should look like this:

```rust
fn main() {
     let name = "Jeff";
     println!("Hello, {}!", name);
}
```

This extremely simple starting point highlights some key components of a Rust program:

- the main() function serves as the entry point to your program
- we bind the string "Jeff" to a variable called "name" (you should change this to be your own name!)
- the println! macro takes the string "Hello, {}!" as an argument, inserts the value of name in place of the curly braces and prints it to the console

The other key items to note are are that Rust expressions must terminate with a semi-colon and expressions and statements within a function are surrounded by curly braces. The curly braces are especially important in the discussion of ownership, a core memory management feature offered by Rust

To run our very basic program, back at the command line, type:

```bash
cargo run
```

You should see the message “Hello, (your name)” outputted to the terminal. Congratulations! No matter how simple, you’ve just programmed in Rust.

That’s it for this very bare bones introduction. The language has a lot of power and there are many concepts to learn and master. More to come in the future!

### Resources:

[The Rust Programming Language](https://doc.rust-lang.org/book/)

[The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
