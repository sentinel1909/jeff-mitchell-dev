+++
title = "Matchy Matchy"
description = "An introduction to match syntax for flow control."
date = "2022-09-30"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "control flow", "match"]
+++

My journey in learning the Rust language continues. Today, let’s explore one of Rust’s unique methods of controlling program flow, known as the match construct. The match syntax is really addictive, and sorely missed when you use other languages.

Previously, I’ve explored the traditional ways of controlling program flow, namely if..else and looping. Rust has another way, the match construct, which allows your code to evaluate and act on multiple possibilities.

Let’s dive in and explore through a fun, silly little Star Wars themed example.

```rust
enum Classification {
    Starfighter,
    Freighter,
    Corvette,
    Frigate,
    StarDestroyer
}
fn get_ship_type(ship: Classification) -> &'static str {
    match ship {
        Classification::Starfighter => "A small fighter-type craft",
        Classification::Freighter => "A craft which carries cargo and things",
        Classification::Corvette => "The ship that carried Princess Leia in Star Wars: A New Hope",
        Classification::Frigate => "Like the ship Luke was on at the end of The Empire Strikes Back",
        Classification::StarDestroyer => "The classic wedge shaped subjugator of worlds"
    }
}
fn main() {
    let xwing = Classification::Starfighter;
    let millenium_falcon = Classification::Freighter;
    let tantive_iv = Classification::Corvette;
    let medical_frigate = Classification::Frigate;
    let devastator = Classification::StarDestroyer;

    println!("An X-wing is: {}", get_ship_type(xwing));
    println!("The Millennium Falcon is: {}", get_ship_type(millenium_falcon));
    println!("The Tantive IV is: {}", get_ship_type(tantive_iv));
    println!("The Medical Frigate is: {}", get_ship_type(medical_frigate));
    println!("The Devastator is: {}", get_ship_type(devastator));
}
```

We first declare an enumeration which creates some different types of space craft. We then declare a function called get_ship_type whose job is to receive a ship type as a parameter and return a string with a descriptive message. The meat of this function is a match construct.

Match has the notion of “arms” which are evaluated one at a time. The first match that is found forms the return value for the function. The other possibilities are ignored until the function is called again with a different ship type. In this case, our function returns a simple static string. For now, ignore the “tick” lifetime modifier on the static str return type, it isn’t important for the topic at hand. I will write about lifetimes in a future article.

Matches must be exhaustive, all the types we set out in our enum must have a corresponding match arm to check against. The compiler will squawk if you don’t handle all the possibilities.

In the main function, we declare variables, one to represent each of our possible ship classifications. Then, one by one, we print out a message, calling our get_ship_type function each time to gather the rest of the message.

Individual arms within each part of the match can contain multiple Rust statements, simply enclose the code within curly braces. This is where the power of match comes, sophisticated possibilities and code blocks can form each arm of the match. The results of a particular match arm an also bind to a variable.

Match evaluations must be exhaustive. We have to evaluate every possible outcome otherwise a compiler error will occur. What happens if we care about one or two patterns and nothing else? The catch-all pattern, \_ , is available. Let’s look at another example:

```rust
extern crate rand;
use rand::Rng;
fn main() {
    let mut dice_roll = rand::thread_rng();
    match dice_roll.gen_range(1..20) {
        20 => println!("Critical hit!"),
        _ => println!("Sorry, you missed."),
    }
}
```

You can copy this code into the Rust Playground and it will compile so that you can see the output.

I haven’t talked about crates yet, but know that we use an external Rust crate which helps generate random numbers. We declare a mutable variable called dice_roll which which contains our random number. Then, we create a random number between 1 and 20 and match on it. If our random number is 20, we print out a success message. If the random number is any other number, then we print out a failure message.

The catch-all is useful in situations where we care about one particular result and others can just have a common outcome.

Match is useful in error handling code. It is also useful in extracting values out of the Some variant within the Option enum type. It’s a little more elegant that plain old if..else and helps your code be more readable and understandable, as you can avoid complex nested logic. There are some advanced concepts that you can incorporate with the use of match, including the notion of “guards”. I’ll discuss these in a future article.

Thanks for reading! I hope you are enjoying your own Rust journey and are feeling more and more empowered by this amazing language.

### References:

[The Rust Book, Chapter 6.2, The Match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
