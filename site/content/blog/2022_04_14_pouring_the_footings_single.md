+++
title = "Pouring the Footings: Single Value Data Types"
description = "A continued study of the basics in Rust, focusing on the single value data types."
date = "2022-04-14"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "data types", "single value"]
+++

Today I continue my writing campaign about the Rust language. If you haven’t noticed, I’m roughly following the chapter layout of The Rust Programming Language, also known as “The Rust Book”. I’ve tried repeatedly to read The Book, but have concluded reading passively is useless. It’s a book that requires active reading.

So, I will semi-blog my way through it here on my site.

I’ve covered the following basics so far:

- creating a Hello, World
- use of constants
- variables, mutability, shadowing, and comments
- I’ll now turn to another basic language building block, data types. Data types are at the core of getting things done in Rust.

There are two broad data types available, single value and compound value. This article will focus on single value types. Compound value types are on deck as they deserve attention on their own.

### Scalar Types

Single value data types represent one value, be it a number, character, or boolean. They are the simplest data types to work with and can be used in lots of ways and in combination with more complex data structures. Grouping these simple data types into structures, vectors, or hashmaps are typical use cases.

_Integers_

An integer is a number without a fractional component and can be signed or unsigned. There are several different maximum lengths of integers, chosen depending on the size of the number you need to represent. Rust defaults to an i32 or 32-bit integer.

Here’s an example of an integer variable:

```
let life = 42;
```

We’ve created a variable named life and bound the integer value 42 to it.

_Floating-Point_

A floating-point number has a decimal component. Rust supports 32-bit (f32) and 64-bit (f64) floating-point numbers. The default is f64. All floating-point numbers can be signed, there is no unsigned option like with integers.

Let’s create a floating-point variable:

```
let pi = 3.14;
```

We’ve created a variable named pi and bound the floating-point value 3.14 to it.

_Numeric Operations_

Rust supports all the basic mathematical operations that one would expect in a programming language. These are addition, subtraction, multiplication, and division. There is also a remainder operation, which provides the remainder when one number is not cleanly divisible by the other.

The combined knowledge I’ve written about in my articles thus far allows us to write a tiny program to do math on integers or floating point numbers. Here’s one possiblity:

```
fn main() {
     let x = 5;
     let y = 6;
     let sum = x + y;
     println!("The sum of {} and {} is: {}", x, y, sum);
}
```

Let’s walk through that:

- declare a main function as the starting point of the program
- create a variable named x and bind the integer value 5 to it
- create a variable named y and bind the integer value 6 to it
- create a variable named sum and use the addition operator to add x and y together and bind the result to the sum variable
- use the println! macro to print out a message with our inputs and the sum

_Boolean_

If you need a true or false value, a boolean is the data type of choice. Booleans are used a lot when controlling flow in a program. Here’s an example:

let liar_liar = true;
We’ve created a variable name liar_liar and bound the boolean value true to it.

_Character_

The final simple single value is a character. Characters are bound to variable names like so:

```
let char = 'Z';
```

The character data type can be used to hold emojis as well as traditional characters. It’s important to remember that character values represent a single character. You may be asking yourself about strings, what about them? Well, strings are a separate and distinct data type from characters. Strings carry many different quirks and just as The Book does, I’ll be treating them separately in the future.

### Conclusion

The basic building blocks in Rust are pretty close to my ancient coding knowledge of a language called Pascal. As such, I’ve found them pretty easy to get acquainted with. This article has provided you with an overview of the basic, single value data types in Rust. These basic types can be composed in many different ways and are the core building blocks of any Rust program. Even advanced, higher-order stuff will have some basic math in it somewhere.

There are lots of subtleties to read about regarding the single value data types, I invite you to consult The Rust Book should you wish to learn more.

References

[The Rust Programming Language, Chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)
