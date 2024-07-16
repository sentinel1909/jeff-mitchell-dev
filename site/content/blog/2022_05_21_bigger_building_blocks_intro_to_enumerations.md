+++
title = "Bigger Building Blocks: Intro to Enumerations"
description = "An introduction to Rust's enum data type."
date = "2022-05-21"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "enums"]
+++

Today I will cover the very basics of Rust’s enum type. Enums are a pillar of the language.

### What is an Enumeration?

An enumeration, or enum, is a type of data structure useful for enumerating possible variants of something. In addition, enums can optionally contain data. Enums are best to describe variations of one particular thing. They differ from structs, as structs can contain multiple things all grouped under one category.

### How Do I Create an Enumeration?

Let’s say we needed a data structure to define several possible variants of peppers. We can do it with an enum:

```rust
enum Pepper {
	Bell,
	Banana,
	Pimento,
}
```

I’ve now defined a custom type that can be used throughout the rest of a program.

Enums can also contain data. Data can be attached directly, without the need for another data structure. We can embody some data about spiciness, by adding the Scoville Heat Unit rating, right into the enum variants, like so:

```rust
enum Pepper {
	Bell(i32),
	Habanero(i32),
}
```

We’ve added an integer data type with each of our peppers so that we can later add in a Scoville Heat Unit value. Each element in an enum can be assigned different quantities and types of data. There is no limitation on the number of variations an enum can contain. I’ve kept it simple for the purposes of this article.

### How Do I Use an Enumeration?

Let’s see how we can use our Pepper enum.

```rust
let sweet = Pepper::Bell(0);

let spicy = Pepper::Habanero(100000);
```

Here, we declare two variables, sweet and spicy, and assign them the types we declared in our Pepper enum. We also assign an integer to embody the Scoville Heat Unit rating.

To access the data contained in our Pepper enums, we have to use the match keyword. I’ve not introduced match yet in my writing, so consider this the first glimpse. I will write more in-depth on it in a future article.

```rust
enum Pepper {
     Bell(i32),
     Habanero(i32),
}

fn main() {
  let sweet = Pepper::Bell(0);
	let spicy = Pepper::Habanero(100000);

	match sweet {
		Pepper::Bell(shu) => {
			println!("The Scoville Heat Unit rating of the bell pepper is: {}", shu);
		},
		_ => ()
	}

	match spicy {
		Pepper::Habanero(shu) => {
			println!("The Scoville Heat Unit rating of the habanero pepper is: {}", shu);
		},
		_ => ()
  }

}
```

This program will result in the following output to the terminal:

```
The Scoville Heat Unit rating of the bell pepper is: 0
The Scoville Heat Unit rating of the habanero pepper is: 100000
```

The match keyword in Rust can be used as a sort of flow control, evaluating all possible options of an enum and providing a custom code path to execute for each one. It’s important, when writing match expressions, that ALL possible variants of an enum must be accounted for. If you don’t, you’ll get a compiler error.

Let’s break down what we did above.

We create a match expression first for the sweet pepper variable we declared earlier. We access the data inside the bell pepper variant by declaring a new variable called shu, to represent the Scoville Heat Unit. The first “arm” of our match will result in the output of a simple statement printed to the console, including the value of the Scoville Heat Unit rating. The second, empty match arm is needed because the enum might not contain a value. Remember that all possible variations must be accounted for. In this case, we simply want to do nothing, hence the use of the unit type (), which returns nothing but satisfies the need to deal with all possible variants. In this made-up code sample, our Pepper enum always contains a value, so after execution of our first match arm, code execution continues on to the spicy pepper case.

We write a second match expression, structured similarly, for our spicy pepper, which outputs its Scoville Heat Unit rating.

### Conclusion

I’ve shown you the very bare bones of Rust’s enum structure, which is useful to create custom data types. Enum’s can optionally contain data, increasing their flexibility. The match keyword is used to access internal data within an enum and use it or take other action based on the internal data. Enums are a core part of the powerful type system in Rust.

There are two special versions of the enum type, Option and Result, which contribute to Rust’s robust error handling ability. I will cover these in a future article.

### References

[The Rust Programming Language, Chapter 6.1.Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
