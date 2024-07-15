+++
title = "Just Say No to Magic Values"
description = "Declaring variables as constants are important in any programming language. In this post we'll take a look at how they're handled in Rust."
date = "2022-04-09"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "constants"]
+++

The journey to proficiency in Rust continues. In a [previous article](@/blog/2022_04_05_baby_steps_with_rust.md), I provided a brief overview of how to get started with Rust by creating a tiny cliche of a program that displays the message, “Hello, World!”.

This article will focus on a specific language aspect, namely constants.

### What is a Constant?

A constant is a value that can be bound to a name, similar to a variable. By its nature, the value cannot change.

### What is the Purpose of a Constant?

Constants are useful in any programming language because they help you avoid so-called “magic values”. A magic value is something that is hardcoded into the code base…likely in a million different locations, by the time all is said and done, which makes it impossible (or at least nasty) to properly update and change the code.

Instead of hardcoded values, a handful of constants declared upfront make it clear and easy to update them later, should their values need to change.

### Declaring Constants

Here’s how to declare the speed of light (in metric) as a constant in Rust:

```rust
const SPEED_OF_LIGHT: u32 = 299792458
```

First, the const keyword is used, followed by a name. The convention is all caps. Next, the constant must receive a type annotation, I’ve used a 32-bit unsigned integer type. The last step is to bind the value to the new constant with the = operator.

### Scope of Validity

Constants can be declared in any scope, including the global scope, as such, they can be made widely available.

### Other Considerations

As one might expect, constants are immutable by default. Their values can’t be changed and you can’t use the mut keyword with them. A key concept in Rust is the notion that variables are immutable by default. One must expressly choose to make them mutable (changeable) by adding the keyword mut to the variable binding. Mut does not work with values that are constant.

Thanks for reading my short but sweet thoughts on contants in Rust.

### Resources

[Rust Programming Language: Common Programming Concepts, Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
