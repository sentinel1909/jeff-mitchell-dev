+++
title = "Let's Get Functional"
description = "An introduction to functions in Rust and their associated syntax."
date = "2022-04-22"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "functions"]
+++

In my last Rusty writings, I covered Rust’s [compound data types](@/blog/2022_04_18_pouring_the_footings_compound.md). Today I will write to you about functions. Functions are essential tool to help make code more organized and maintainable.

### Purpose

I’m sure you are familiar with the term spaghetti code, in which your code is a nested mess of twisted logic. Functions are a partial solution to this as they encourage modularity and can make your code base more understandable. Common tasks can be moved to separate blocks of code, then called upon by the main program when needed.

### Syntax

We’re already familiar with one function in Rust, the main function. It serves as the entry point to your program. Every Rust binary program has one.

```
fn main () {
     // program logic goes here
}
```

A function is declared with the fn keyword followed by the function name. Immediately after the name are brackets to contain possible parameters. Then, if the function is to return a value, the → syntax is used with a type declaration. Lastly, we have curly braces denoting the beginning of the code block comprising the function.

Functions in Rust uses the so called snake case as the naming convention. The snake case consists of two or more lower case words separated by an underscore.

Let’s try it:

```
fn remaining_lyrics() {
     println!("...give me that which I desire!");
}

fn main() {
     println!("Give me fuel, give me fire...");
     remaining_lyrics();
}

Output:
Give me fuel, give me fire...
...give me that which I desire!
```

What have we done here?

- declared a new function called remaining_lyrics
- the function accepts no parameters and doesn’t return anything
- the function uses the println! macro to print out a message to the terminal
- declared a main function
- the main function prints out a message to the terminal
- the remaining_lyrics function is called to finish the message

One item of note is I’ve declared the function first before the main function. This is just a choice on my part. Historically I’ve always liked declaring my functions first, then having the main function as the very last thing. This is just a personal preference.

### Parameters

Functions can accept values, also known as parameters, on which to do further operations. Parameters are sometimes referred to as arguments. You may see these terms used interchangeably in your coding travels. I will try to be consistent going forward and refer to them as parameters.

Let’s adapt our snippet above to use some parameters:

```
fn remaining_lyrics(band: &str, album: &str, song: &str) {
     println!("...give me that which I desire! {}, {}", band, album, song);
}

fn main() {
     println!("Give me fuel, give me fire...");
     remaining_lyrics("Metallica", "Reload", "Fuel");
}

Output:
Give me fuel, give me fire...
...give me that which I desire! Metallica, Reload, Fuel
```

We’ve now made our remaining_lyrics function accept 3 string slices as parameters. It accepts these string slices when they are passed from the main program by placing them inside brackets right after the function call. The output is the same, but now we have Metallica, Reload, and Fuel printed after the lyrics.

An essential thing about using parameters in functions: the type of the parameter must be annotated to the parameter name. In our little music example, the &str type has been annotated. Failing to annotate the type will result in a compiler error.

### Return Values

Functions can not only receive parameters and process them. Functions can return values back to where they are called from. Let’s make a program which prints a message to tell us the year of release of a very famous album by a favourite band:

```
fn year_released() -> u32 {
    1991
}

fn main() {
     println!("The Black Album, by Metallica, was released in: {}",     year_released());
}

Output:
The Black Album, by Metallica, was released in: 1991
```

Here, the main function prints out a simple message, but calls the function year_released(), which accepts no parameters, and simply returns an integer. This example serves to illustrate an important concept, beyond just the notion of returning a value, the difference between statements and expressions.

### Expressions vs Statements

Rust functions are made up of statements and expressions. Statements are instructions that perform some action and do not return a value. An example is the println! statement in the remaining_lyrics function above. It performs the action of printing out a message. Expressions evaluate out to a value. This is what happens in our example above of returning a value. The year_released() function has a single expression which evaluates to an integer.

- Statements are terminated with semi-colons.

- Expressions are not terminated with semi-colons.

Use of the return keyword at the end of a function is optional in Rust. There are times when it’s beneficial to use the return keyword, such as in the instance of an early return out of a function.

### Conclusion

This article has covered the basics of functions in Rust. It’s shown you the syntax as well as explained the ability of functions to accept optional parameters and return values. Functions are the bread and butter of Rust and provide a means of keeping your code concise, tidy, and maintainable. In my own journey to learn Rust, I’ve found functions relatively easy to understand, as their structure is almost identical to other languages I have familiarity with, namely C and Turbo Pascal (wow, I’m old). I hope you’ve enjoyed this short article on functions. Next up will be decision making and flow control. See you next time!

References

[The Rust Programming Language, Chapter 3.3](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
