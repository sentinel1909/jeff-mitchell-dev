+++
title = "Method to the Madness"
description = "An introduction to the method syntax."
date = "2022-07-01"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "methods"]
+++

### The Method Syntax

I’m going to turn my attention now to something called the method syntax. It’s common to structs and enums, which I’ve covered in previous outings.

Structs and enums can have methods associated with them. This enables custom functionality to be implemented.

Let’s do this discussion with the help of an example.

### A Circle Calculator

Here is a Rust program which will help us calculate the key characteristics of a circle, namely its area, diameter, and circumference. Respectively, these characteristics are:

- area: the amount of space inside the circle, the equation for calculating the area is: A = pi x r²
- diameter: the “width” of the circle, the equation for calculating the diameter is: D = 2 x r
- circumference: the distance around the circle, the equation for calculating the circumference is: C = 2 x pi x r
- r is the radius of the circle and pi is the Archimedes’ constant which is 3.14

```rust
use std::f32:consts::PI;
struct Circle(f32);

impl Circle {
	fn area(&self) -> f32 {
		PI * (self.0 * self.0)
	}

	fn diameter(&self) -> f32 {
		2.0 * self.0
	}

	fn circumference(&self) -> f32 {
		2.0 * PI * self.0
	}
}

fn main() {
	let circle1 = Circle(15.0);
	println!("The area of the circle is: {} metres", circle1.area());
	println!("The diameter of the circle is: {} metres", circle1.diameter());
	println!("The circumference of the circle is: {} metres", circle1.circumference());
}
```

We need the constant pi, which is available to us in Rust’s standard library, so we pull this in as a dependency with a use statement. Next, we declare a typle struct called Circle. This tuple struct contains one value, a 32-bit floating-point number which represents the radius. Next, we get to the meat of the matter. Structs can have methods defined for them, which allow you to write code to do something with the struct. Methods are started with the impl keyword, followed by a name which must match the name of the struct. Inside the curly braces, we define one or more functions which comprise our “implementation block” for the Circle struct. Everything inside this block is associated with the Circle struct.

The first function, called area, takes a reference to the struct itself as a parameter (still haven’t talked about references and borrowing yet…I will get there, I promise) and returns the area of the circle. The equation I gave above for area is used. To access the radius value contained within our struct, we use self.0 to reference it. It’s important to note that Methods must take self as their first parameter. The can have other parameters, but self is always the first.

The diameter and circumference functions perform a similar task, taking the radius provided as part of the struct, using the equations I noted earlier, and returning the diameter and circumference.

The main function ends up being very lean, given we’ve implemented most of the program through the methods on the Circle struct. We declare an instance of our Circle struct, called circle1, and give it a radius value of 15.0 metres. Then, we calculate and output our results by calling, in turn, each of the methods we created. Methods are called by simply appending the method’s name to the struct variable, using dot notation.

The output from this program will be:

```rust
The area of the circle is: 706.5 metres
The diameter of the circle is: 30 metres
The circumference of the circle is: 94.2 metres
```

I’ll leave you to embellish this program with things like user input, maybe from command line arguments using the clap crate or whatever other way you choose. You could also fancy up the output if you like.

### Conclusion

I’ve presented the method syntax to you with a simple, practical example. Methods are a means of providing encapsulation and organization to your Rust programs. They enable the selective exposure of functionality to users as well as other parts of the program. Everything related to Circles is in one place, making it straightforward to know how to interact with the program.

### References

[The Rust Programming Language, Chapter 5.3, Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
