+++
title = "Building Something - Saying Hello"
description = "An example program which demonstrates how to obtain user input, transform it, and output the result."
date = "2022-05-06"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "example", "input and output"]
+++

I’ve covered what I consider the core concepts of the Rust language, namely:

- constants
- comments, variables, mutability and shadowing
- simple data types
- compound data types
- functions
- decision making
- flow control
- From this core, useful things can arise. Let’s build something.

In my Let the Journey Begin piece, I showed you how to create the classic coding journey starting point, a hello world program. That little grain of salt didn’t take any user input. We shall write a program which asks for your name and then prints to the console a simple greeting.

Here’s what the output will be:

```
What is your name? Jeff
Hello, Jeff, nice to meet you!
```

There are a few important concepts that will bubble up from this simple example. You’ll learn how to use Rust’s standard library as well as how to “sanitize” or clean up input received from a user. We’ll also take the opportunity to incorporate modularity by breaking the program up into distinct parts, each with a specific purpose.

### Problem Statement

The “Hello, World” program is the first program you learn to write in many languages but doesn’t typically involve any input. Create a program that prompts for your name and prints a greeting using your name.

### Constraints

We’ll impose some constraints on our solution and exercise the principle of “separation of concerns” to make our program modular. We’ll structure our program into separate functions to keep the input, string building, and output jobs separate.

### Walkthrough

I’m terrible for just jumping in and coding, without solving the problem or outlining my program structure. I vow to help you be better. Let’s think for a minute about what we need to do.

- ask for the user’s name
- “sanitize” the user’s input, as it will contain white space (a carriage return) that will affect our output format
- build the greeting string from three parts, including the input received
- output the result

_Function to get the name_

```
fn get_name() -> String {
     print!("What is your name? ");
     io::stdout().flush().unwrap();
     let mut buffer = String::new();
     io::stdin().read_line(&mut buffer).unwrap();
     buffer
}
```

First up is our get_name function. It doesn’t need to take any parameters and must return a string to the next part of our program. We use the print! macro to output the text “What is your name?” and then wait for input. We haven’t used println! because we don’t want a carriage return. We need to pose the question, then wait for the user to type it all on the same line.

Next, we declare a variable named buffer, which is made mutable so that we can receive the input and store it. We take the user’s typed name, store it in our mutable buffer variable, and then the function ends and we return the name as a string.

You’ll notice the .unwrap() method. I’ve not explained this yet, but will in the future. There’s a good preview here at [Rust by Example](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html). The read_line method might fail, in that it might not return a result. The use of .unwrap() says we’ll ignore the possible error and assume we get the result we need. The program will panic and crash if we don’t get the name as an input.

There is a concept glossed over. The use of the print! macro carries with it the need to force out our text of the request to enter your name, as it may not be emitted immediately. We use the .flush() method on io::stdout() to achieve this. Similar to the use of the read_line method, we .unwrap() the result and ignore the potential for errors.

_Function to build the greeting message_

```
fn build_greeting(name: String) -> String {
     let mut greeting = "Hello, ".to_owned();
     greeting.push_str(name.trim());
     greeting.push_str(", nice to meet you!");
     greeting
}
```

In the build_greeting function, the main work happens. This function needs a parameter, namely the name string we just received from the get_name function. A string (the complete greeting) will be returned. We declare a mutable variable (an owned string…haven’t talked about what ownership means yet, but it’s coming soon). We use the push_str method to append our name variable and we use the trim() method to trim off any carriage returns received as part of the name variable. This action serves to “sanitize” the user input. Then, we append the last part of the greeting, once again using the push_str method. Finally, we return the completed greeting.

_Function to print the greeting to the console_

```
fn print_greeting(greeting: String) {
    println!("{}", greeting);
}
```

Our final function, print_greeting, receives our greeting string as a parameter and uses the println! macro to output it to the console.

### Final Code

Let’s put it all together.

```
use std::io::{self, Write};
// function that retrieves user input
fn get_name() -> String {
     print!("What is your name? ");
     io::stdout().flush().unwrap();
     let mut buffer = String::new();
     io::stdin().read_line(&mut buffer).unwrap();
     buffer
}
// function that builds the output string
fn build_greeting(name: String) -> String {
     let mut greeting = "Hello, ".to_owned();
     greeting.push_str(name.trim());
     greeting.push_str(", nice to meet you!");
     greeting
}
// function that displays the output string
fn print_greeting(greeting: String) {
     println!("{}", greeting);
}
// main program
fn main() {
     print_greeting(build_greeting(get_name()));
}
```

The very beginning of our program has a use statement. We need to use some things from Rust’s standard library (stdin and stdout, along with their associated methods), to aid in getting our input, and we need to bring them “into scope” so that our program can find and use them.

The main function calls the three functions in a nested fashion, with the result being our greeting to the user. In this nested function call, it’s important to remember that the get_name function returns the name, which serves as the parameter that the build_greeting function needs. Lastly, the greeting string returned from build_greeting serves as the parameter that print_greeting needs to print the greeting.

### Conclusion

I’ve shown you a step-by-step methodology for creating a program which asks for user input in the form of your name, builds a greeting using by combining the name with a pre-built string, and outputs the result to the console. This program is simple but surfaces several subtleties that are useful going forward and act as building blocks to the creation of bigger and better things. It’s critical to imagine the flow of data through your program and the act of working through a problem with a simple solution helps sharpen your ability.

GitHub Repo

I’ve put the code into a GitHub repo, you can access it here:

[57-exercises: Say-Hello](https://github.com/sentinel1909/57-exercises)

References

Exercises for Programmers: 57 Exercises to Challenge your Programming Skills, by Brian P. Hogan

[The Rust Standard Library: std::io](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/io/index.html)
