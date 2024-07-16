+++
title = "Result-ful Thinking"
description = "An introduction to the Result type in Rust."
date = "2023-04-14"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "enums", "result"]
+++

The Result enumeration in Rust is the second special enumeration available in the standard library. It is a powerful tool for error handling and enables you to handle errors in an elegant, recoverable fashion. Using the Result enum is a bit of a shift in thinking from other languages.

The Result enum has the following signature:

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

I've yet to write a lot about generics yet, but similar to the Option enum, Result has two generic type parameters. The T represents the type that will be returned in a success case, it's wrapped in a n "Ok" variant. The E represents and error type that is reutrned in a failure case, it's wrapped in an "Err" variant.

Let's revisit the contrived example of a program that divides two numbers, as I used in the article on the Option enum:

```rust
fn divide (numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
		Err("Cannot divide by zero".to_string())
	} else {
		Ok(numerator/denominator)
	}
}

fn main() {

	let numerator = 10.0;
	let denominator = 2.0;

	let answer = divide(numerator, denominator);

	match answer {
		Ok(ans) => println!("Result: {}", ans),
		Err(err) => println!("{}", err),
  }
}
```

In our main function, we declare a numerator variable and a denominator variable. We call the divide function, sending in the numerator and denominator as parameters. The divide function returns a Result, which will either contain our answer, or an error message. The body of the function uses an if statement to establish the return value, be it error or answer. If the denominator passed in as input is zero, this generates a string stating "Cannot divide by zero". The main function does a match on the answer value returned by the divide function. We either print out the answer, or print out our error message.

Handling errors in this way is very elegant. Rather than just throw out an error which could be not very meaningful, we can actually write code that explains very specifically what the issue is. The end result is code which is far more robust.

Want to know more about the Result type? Head over to official Rust Programming language book and [read more](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
