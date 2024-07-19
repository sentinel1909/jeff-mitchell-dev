+++
title = "The Memory Doesn't Remain"
description = "An introduction of how memory is handled in Rust, including the concepts of ownership and borrowing."
date = "2022-07-14"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "memory", "ownership", "borrowing"]
+++

> "Fortune, fame, mirror vain, gone insane...but the memory remains!" - Metallica, The Memory Remains

This is the article I’ve been needing to write for about a month (maybe two) but have fretted and procrastinated.

Memory, ownership, borrowing…oh my!

These concepts are probably the most difficult things in Rust. I worry that I’m not doing these topics justice. However, I’m going to shove this worry aside and just go for it. I intend on returning to this article periodically to update it as my understanding improves.

If you never start, you never finish, so here we go…

### Dealing with Garbage

In programming languages, there are two broad approaches to handling the use and cleanup of memory.

The first is manual, as in it’s all on you the programmer. An example is the C language. The C compiler leaves memory use entirely up to you. If you forget to free memory after you’ve finished with it, an error may result because the C compiler is blissfully unaware. More than likely, you’ll be dealing with the consequences later.

The second is so-called garbage collection. Languages such as JavaScript use a built-in memory manager, called a garbage collector, that follows along behind you to clean up memory. This brings overhead, which may be unacceptable for resource-constrained systems.

Rust introduces a third way. The Rust compiler brings a set of rules that are checked at compile time. Code will not compile if any of the rules are broken. The rules do not impact the performance of your program because they are enforced at compile time.

Before we get into the nitty gritty of this seemingly magical system, there are a couple of other general concepts to review.

### The Stack vs. The Heap

Like C, Rust works within the same two memory regions, the stack and the heap.

#### The Stack

Anytime you create a variable using Rust’s simple types, like an integer, the variable is created on a region of memory known as the stack. Things that go on the stack have to be fixed in size and can’t change. Fresh data is always stored at the top of the stack. Storing on the stack is quick. I recommend the ownership chapter of the Rust Book, which provides a good overview of the stack.

#### The Heap

The heap is a dynamic area of memory that can grow and shrink as needed. When data is placed in the heap, the memory allocator finds a contiguous block of space and returns a reference called a pointer. Accessing memory on the heap is slower than on the stack, because, in effect, it needs to be looked up first. I recommend the ownership chapter of the Rust Book, which serves as a good overview of the heap.

### Ownership and Scope

Rust has three rules of ownership:

1. Each Rust value has a variable called its “owner”.
2. Each value can have only one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

To begin thinking about ownership, it’s essential to think about scope. A scope is denoted by curly braces. Here’s an example:

```rust
fn main() {

    let fuel = "Give me fuel, give me fire";
}
```

We declare a string literal and bind it to a variable called fuel. Variable fuel comes into scope as it’s declared. We can do what we need with it right up until the closing curly brace. Past that point, Rust calls a special internal drop function to clean up the trash. The memory that our fuel variable used is freed up.

Any variable in the Rust language is valid when it comes into scope and it remains valid until it goes out of scope. The compiler will throw an error if you attempt to access the value in a variable that has gone out of scope.

A consequence of this notion of validity until the scope ends, where functions are concerned, is that we have to pass in AND pass back anything that we want to use later. If we don’t the values will be lost.

```rust
fn main() {
	let fuel = String::from("Give me fuel, give me fire");

	let (fuel2, len) = calculate_length(fuel);

	println!("The length of '{}' is {}.", fuel2, len);
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();

	(s, length)
}
```

Here, we declare a variable called fuel and bind the same string literal as above to it. We then declare a tuple, consisting of a new string variable and an integer (to represent the string length that we’re going to determine) and we bind to the tuple returned by the function calculate length. We pass in our fuel variable to this function.

The function takes our passed string and returns it along with its length. The len() method is used on our passed-in string to determine the length. We finally print the string and its length to the console.

This is a lot of work. We need to do a lot of moving and passing around values to make sure we don’t lose them due to variables going out of scope. Fortunately, we’re saved by the notions of references and borrowing.

### References and Borrowing

The second pillar of Rust memory management is the notion of a reference. We can use values owned by other variables without claiming ownership. This is called borrowing.

What if we could simply reference values in variables without taking ownership of them? Let’s revisit our previous example.

```rust
fn main() {
	let fuel = String::from("Give me fuel, give me fire");

	let len = calculate_length(&fuel);

	println!("The length of '{}' is {}.", fuel, len);

}

fn calculate_length(s: &String) -> usize {
	s.len()
}
```

This code may not seem simpler, but it is. We declare a string variable and give it a value as before. Next, we call a function to get the string length, but this time we pass in a reference to our fuel variable. The & represents a reference, allowing us to refer to a value without taking ownership of it.

You’ll notice the function signature simplifies as it now takes a reference to a string and returns a usize, which is basically an integer. Inside the function, we again get the length of the string using the len() method.

We have just borrowed the value of another variable to do something with it.

You’ll note here that we don’t need to return the value from the function calculate_length, to give back ownership, because change never had ownership in the first place.

### Mutability and Borrowing

References are immutable by default. We can’t change the value of something we don’t own. To change the value referenced, we have to use a mutable borrow. Let’s look at an example:

```rust
fn main() {
	let mut fuel = String::from("Give me fuel, give me fire");

	change(&mut fuel);
}

fn change(some_string: &mut String) {
	some_string.push_str(", give me that which I desire");
}
```

We declare a mutable variable called string and we bind our favourite song lyrics to it. The fuel variable can change because we made it mutable. Next, we call a function called to change and pass it a mutable reference to our fuel variable. Ownership will not be transferred, and the value of fuel will not be moved to the function, because we are simply borrowing this variable. The change function takes a string as a parameter, which is annotated as a reference to a mutable String. The function body appends some text to the string to complete our song lyrics. Once again, the function makes our change but does not have to pass anything back.

### Conclusion

The concepts of ownership and borrowing are difficult to get used to. There is a lot of so-called “fighting the borrow checker” which comes with the early days of learning in Rust. However, over time the concepts do become relatively intuitive. Rust’s rules of ownership combine to make Rust’s code inherently safe, performant and free of the issues that plague languages like C. The compiler checks for us, giving us confidence that our code is, out of the gate, safe and trustworthy.

### References

[The Rust Programming Language, Chapter 4.1, What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

[The Rust Programming Language, Chapter 4.2, References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
