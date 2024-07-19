+++
title = "Pouring the Footings"
description = "An overview of basical foundational concepts, including comments, variables, mutability and shadowing."
date = "2022-04-12"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "comments", "variables", "mutability", "shadowing"]
+++

Before I continue in my series of articles on learning Rust, I’d like to remind you that I’m a random internet nobody. I’m far from an expert in this language and feel I have no right to be writing about anything to anyone most of the time. Learning to code is difficult and there are probably far better-informed experts to learn from than me. However, I have the tiny hope that sharing my journey will be beneficial to someone. There are precious few people writing about their experience with Rust. The act of writing about my learning process and the hurdles I overcome benefit me and I hope you do as well.

Enough navel-gazing, let’s get on with some more foundational concepts in Rust. These building blocks are crucial. I know, speaking for myself, I’m in a terrible race to understand the higher-order concepts, because those are what make you able to build things other than toy software. The truth is that higher-order concepts are meaningless without solid footings to rest on. I’ve already written a short piece about Getting started with a Hello World app as well as a short piece on the advantages of Constants, so now I’ll turn to comments, variables, mutability, and shadowing.

### Comments

I’m going to start by asking you to take my advice because I’m not using it. I say that because I’m terrible at commenting on my code. Comments are important, don’t be like me. You should liberally comment on your code. Think of comments as bread crumbs to your future self. You could struggle for days on a particular piece of your program and if you’re diligent with commenting, you’ll have something to come back to later when faced with a similar problem. Comments are a great way to document the problem-solving process. Try to make them reflect the thinking in getting to the code, not so much what the code does.

Comments are prefaced with two slashes, like so:

```rust
// This is a Rust comment.
```

Multi-line comments are done like so:

```rust
// This is the first line...
// ...of a multi-line comment.
```

Comments can go anywhere you like, but tend to be most useful right around what they’re commenting on.

```rust
fn main() {
     // This is a great place for a comment. Let's bind an integer
     // to the variable named life, we'll then print out to the
     // console the value of the meaning of life.
     let life = 42;
     println!("What is the meaning of life? {}", life);
}
```

There is a third type of comment, which powers as an aspect of the Rust that I find amazing. The third type is a documentation comment. It looks like this:

```rust
/// This is a documentation comment in Rust.  These comments are picked up by Cargo's automated documentation tool.
```

I’ll write more about this down the road. If you want to read more now, head over to Publishing a Crate to Crates.io in the Rust Book.

### Variables, Mutability, and Shadowing

_Assigning Variables_

Programming languages would be all but useless without variables. Variables allow the creation of programs that can accept a wide variety of input to do some action. In Rust, the act of creating a variable and giving it a value looks like this:

let album = “Attero Dominatus”;
Here we bind a string slice (more about that in another article) to a variable named album, using the let keyword.

_Immutability of Variables_

If you write some code that re-assigns the album variable later in your program, you’ll receive a compiler error that informs you that you can’t assign twice to the immutable variable ‘band’. A pillar of Rust is that variables are always immutable, meaning they can’t be changed after assignment. This protects you from a whole range of bugs involving variables that change when least expected. If a variable does need to change, then use the mut keyword to make it mutable.

```rust
let mut band = “Sabaton”;
```

This will allow you to change the value assigned to the variable band, if needed, elsewhere in the program. Default immutability is one of Rust’s terrific safety features and helps you protect yourself by making you think carefully about how and when variables need to be modified.

_Shadowing of Variables_

Rust allows one variable to be shadowed by another. Here’s an example:

```rust
let x = 5;
let x = x + 1;
println!(“The value of x is: {}”, x);
```

Let’s break it down…

- first, bind the value 5 to the variable named x
- second, shadow the value of x, take the original value (5) and add one
- the value of x will be 6 and this gets printed to the console

Shadowing variables is a useful way to transform values from one type to another as well as perform operations on them. The variable is still immutable, even after the completion of any transformations.

### Conclusion

In this article, I’ve hopefully given you some foundational breadcrumbs in the Rust language. Take these and go off to do your reading, research, and practice. As always, I recommend The [Rust Programming Language](https://doc.rust-lang.org/book/) for further reading and detail.
