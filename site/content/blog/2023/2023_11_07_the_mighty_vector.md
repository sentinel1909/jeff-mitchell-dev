+++
title = "The Mighty Vector"
description = "An article introducing the vector collection type in Rust."
date = "2023-11-07"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "collections", "vectors"]
+++

## Indtroduction

Today I resume my journey through the Rust Book, in the spotlight is the vector type.

The Rust Book introduces vectors as a "collection". A vector is similar to an array type, but with the critical difference that a vector can grow and shrink in size. A vector is capable of storing a list of items in a single data structure. All the individual values in a vector are stored next to each other in memory. Vectors can only store data of the same type.

I called this article "The Mighty Vector" because vectors are an extremely versatile and useful way of storing data. I think you'll use them quite a bit in your own Rust adventures.

## Creation

### Vec::new() Function

A fresh, empty vector can be created like so:

```rust
let origin_coordinates: Vec<i32> = Vec::new();
```

Since we haven't initialized this coordinates vector with values, we have to tell the compiler what we want, because otherwise it won't know. The vector type provided by the Rust standard library is implemented using generics and can hold any type. In this case we've said that our coordinates vector is going to contain i32 elements.

### vec! Macro

More often than not, we want to initialize a vector with some values. Rust gives us a macro, the vec! macro, which creates a vector with whatever values we choose:

```rust
let origin_coordinates = vec![0, 0, 0];
```

The initial values allow the compiler to infer what we want, so we don't have to use a type annotation like in the first example.

### Modifying a Vector

Hopefully you recall that all variables in Rust, when declared, are immutable, they can't be changed. If we know we need to change the values in our vector, we need to use the mut keyword:

```rust
let mut coordinates = vec![1, 3, 5];
```

Then, we can add values to this vector by using the push method:

```rust
coordinates.push(10);
coordinates.push(15);
coordinates.push(20);
```

Let's make a complete program to see what we get:

```rust
fn main() {
    let mut coordinates = vec![1, 3, 5];
    coordinates.push(10);
    coordinates.push(15);
    coordinates.push(20);

    for coordinate in coordinates {
        println!("{}", coordinate);
    }
}
```

Here we initialize our vector with some default i32 values, then we push 3 more values into the vector. Finally, we use a for loop to print out the values to the console. Note that we don't need any type annotations because the Rust compiler can infer from the information we've provided.

```bash
Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/playground`

 1
 3
 5
 10
 15
 20
```

### Reading the Elements in a Vector

So, we can add elements to our vector, how do we read the elements back? Rust gives us a couple of ways, depending on what we want our program to do.

_Panic Attack_

The first way of reading an element is by simple indexing.

```rust
let coordinates = vec![25, 24, 23];

let z: &i32 = &coordinates[2];
```

In one of the greatest quirks of computer science, which to this day trips just about everyone up at one time or another, vectors are indexed starting at zero. In the previous example, the indices of our coordinates vector are 0, 1, 2. To get the third element, let's call it the 'z' coordinate, we use & and [] along with the index 2, to give us a reference to the element 23.

This is all fine and happy if the element at the index we request exists. What if it doesn't? If it doesn't, the program panics and immediately terminates. We might want that behaviour, so it is a legitmate option.

_More Elegant Way_

There is another more elegant way to handle the possibility of a vector element not existing:

```rust
let coordinates = vec![25, 24, 23];

let z: Option<&32> = coordinates.get(2);
match z {
    Some(z) => println!("The z coordinate is {z}"),
    None() => println!("Oops, no third element exists in this vector!);
}
```

The .get() method, when passed an index that is outside of the vector's range, leverages the Option type and returns a None value without the panic. You can then use the match statement to gracefully handle the possibilities. This approach results in more user friendliness than a panic and crash, because you can craft error messages to explain what happened.

### Enums to Store Multiple Types

Remember earlier I said that vectors can only hold data of the same type? Well, I lied a teeny tiny bit. We can leverage Rust's enum type to get around this limitation. There are times when we may want to have a list of items that have different types.

```rust
enum SportsTeam {
    Name(String),
    Conference(String),
    Standing(i32),
}

let teams = vec![
    SportsTeam::Name(String::from("Seattle Seahawks")),
    SportsTeam::Conference(String::from("NFC West")),
    SportsTeam::Standing(2)
];
```

This is not the greatest example, because the variations represented by the enum SportsTeam are not that dramatically different. However, it illustrates that we can create a vector to hold some information about our sports team and because the underlying type of each vector element is an enum, this satisfies the need that the elements all be of the same type.

A vector is stored on the heap. The Rust compiler must know exactly how much memory space to allocate at compile time. A match expression needs to be used which aids the compiler in ensuring that every possible variation is handled. If the data in your program is such that you can't know the exhaustive set of types at runtime that a vector will contain, then this enum technique won't work. The solution is to use a trait object, which I'll cover in a future article.

## Conclusion

This article has been my take on vectors. I've gone over the basics, but be sure to check out the Rust Standard Library documentation for the std::vec module, as there are more things you can do with this powerful and flexible data type.

Thanks for reading!

## References

[Rust Standard Library, Module std::vec](https://doc.rust-lang.org/std/vec/index.html)

[The Rust Programming Language, Chapter 8.1 Storing List of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
