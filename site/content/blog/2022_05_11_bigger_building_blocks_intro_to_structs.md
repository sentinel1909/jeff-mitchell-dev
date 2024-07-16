+++
title = "Bigger Building Blocks: Intro to Structs"
description = "An introduction to Rust's struct data type."
date = "2022-05-11"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "structs"]
+++

I’m roughly following the order of The Rust Book in my writings, but I intend on deviating a little. The next topic is ownership and borrowing, but I don’t feel like I’m quite able to write about those two things coherently yet. Instead, I will write about the first of Rust’s bigger building blocks — structs.

### What is a Struct?

A struct is a flexible data structure that allows creation of different data fields within one group. Structs are similar to tuples but more flexible. Each data field is named and can be accessed directly. There is no need to rely on the order of the data in a struct. Named fields serve to make it more clear the overall purpose and details embodied by the struct.

### How Do I Construct a Struct?

Let’s look at how we construct a struct (see what I did there…):

```
struct Album {
	id: u32,
	genre: String,
	artist: String,
	name: String,
	release_date: String,
}
```

We create a struct by using the struct keyword immediately followed by a name. Convention is to make struct names capitalized. We then surround the individual fields with curly braces. The fields of the struct are created with key: value pairs separated by commas. The example here contains predominately string types, with a lone 32 bit unsigned integer. Struct fields can be of any type.

I will highlight one side topic here. The majority of the fields in our struct are Strings, in this case we’ve made them owned strings, meaning our struct will have ownership of all its data. More on ownership will come in a future article.

### How Do I Use a Struct?

It’s important to remember that all we’ve done so far is create the template for a struct. The template has a name, Album, as well as several empty fields. This is a blueprint, if you will, of our data structure. To actually make it useful, we have to add flesh the bones.

We do that like so:

```
fn main() {
	let album1 = Album {
		id: 1,
		genre: String::from("Heavy Metal"),
		artist: String::from("Iron Maiden"),
		name: String::from("Senjutsu"),
		release_date: String::from("September 3, 2021"),
	};
}
```

Let’s unpack this.

We’ve started our program (with the main function) and immediately declare a variable called album1. This variable is our first “instance” of our Album struct. Inside the curly braces we assign specific values to the data fields that are unique to the album1 variable.

It’s important to note that, as declared in this example, the fields cannot be modified. To modify one or more of them the mut keyword is needed as part of the variable declaration.

```
fn main() {
	let mut album1 = Album {
		id: 1,
		genre: String::from("Heavy Metal"),
		artist: String::from("Iron Maiden"),
		name: String::from("Senjutsu"),
		release_date: String::from("September 3, 2021"),
	};

	album1.name = String::from("The Book of Souls");
	album1.release_date: String::from("September 4, 2015");
}
```

This change now lets us re-assign values to the fields of the struct, as illustrated by the reassignment of two of the fields immediately after the variable declaration.

How do we access the fields inside a struct?

Like so:

```
println!("The name of album1 is: {}", album1.name);
```

We call the name of the struct variable, then use dot notation and append the name of the field we want to work with. In this case, we’re outputting the album1 name field to the console using the println! macro.

### Conclusion

Structs are a powerful and often used data structure in Rust. They are a core part of the type system that makes Rust so flexible. In this article I’ve shown you how to create, populate, and use fields inside a struct. There are more things to talk about for structs, but I’ll save those for a future “advanced” article.

### References

[The Rust Programming Language, Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
