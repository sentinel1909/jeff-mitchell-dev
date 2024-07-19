+++
title = "Controlling the Flow"
description = "An introduction to the flow control tools and syntax in Rust."
date = "2022-05-01"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "control flow"]
+++

Last time, in [Decisions, Decisions](@/blog/2022/2022_04_27_decisions_decisions.md), we looked at a method for adding logic within a program, via the if and if else expressions. Next up, we see how we can further control the flow, through loops. Loops allow repetition of blocks of code until some desired event or outcome. There are three kinds of loops in Rust: loop, while, and for.

### loop

The loop keyword tells Rust to execute the same block of code over, and over, and over again, until told specifically to stop. Here’s a basic example:

```
fn main() {
	loop {
		println!("Help, I'm stuck in a loop!");
	}
}
```

This program will endlessly print out the message, “Help, I’m stuck in a loop!”, over and over to the console until you hit the control-C key combo to break out. This is not particularly elegant or useful. Luckily, you can control how long you stay in the loop.

Here’s another example:

```
fn main() {
	let mut count = 10;
	loop {
		println!("Ok, in a loop, but will get out once the count reaches {}", count);
			count -= 1;
			if count < 0 {
				println!("Yay, I'm saved!");
				break;
			}
	}
}
```

We declare a counter variable and bind the starting value of 10 to it. Then, we enter a loop. In the loop, we iterate through the following steps:

- print a message about being stuck in a loop
- decrease the counter value by one
- check if the counter is less than zero, if it is, we print one final message then break out of the loop, terminating the program
- if the counter is not less than zero, around we go again

Values can be returned out of loops. This is useful in the case where you might be computing something, then need to return the value out to the rest of the program. When doing this, the loop can be assigned to a variable, which will hold the result computed from the loop. Here’s an example:

```
fn main() {
	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};

	println!("The result is {}", result);
}
```

Here, you can see that we’ve added the final result of the counter variable to the break keyword, then bound the whole loop structure to the variable result.

### while

Frequently, we need to repeat an action while a certain condition is true. Without any other methods, we could do this with a combination of the loop we just learned about, along with if, else, and break, however, this is cumbersome. Rust has the while loop for us.

Here’s an example:

```
fn main() {
	let mut number = 10;
	while number != 0 {
		println!("Counting down...{}!", number);
		number -= 1;
	}

	println!("Liftoff!!!");
}
```

We declare a variable named number and bind the value 10 to it, when we enter the while loop, checking that number is not equal to 0. We print a message with the value of number, decreasing it by 1 on each pass through the loop. Eventually, we reach 0 and the final message is output.

### for

The last basic loop structure is a for loop. In a for loop, execution happens for as many times as you specify. Any other decision logic you have to add yourself. The for loop will execute the specified number of times, in absence of anything else.

Let’s say we have a couple of arrays and we need to print out all the elements within each. We can do that with a for loop.

```
fn main() {
	let array_one = [40, 80, 120];
	let array_two = [30, 70, 110];

	for array_item in array_one {
		println!("the value is: {}", array_item);
	}

	for array_item in array_two {
		println!("the value is: {}", array_item);
	}
}
```

This will loop through the individual entries in each array, and print the entries to the console.

A handy thing to do with for loops is to pull in Range from the standard library AND use the rev method available on Range to countdown. Let’s do another countdown using this technique:

```
fn main() {
	for number in (1..10).rev() {
		println!("{}!", number);
	}
	println!("We have liftoff!!!");
}
```

This is quite elegant.

### Conclusion

These loops, when combined with decision logic such as if and else if, allow programs to have a more sophisticated execution order and be more flexible to internal and external input. These basic building blocks form the basis of all you do with Rust going forward. When combined with Rust’s more advanced features and data structures, you’re able to create truly useful software. My writings up to this point have covered the very basic programming concepts included in Rust. Before I move on to the more advanced territory from the Rust Book, in my next article I’ll demonstrate building something simple with all that I’ve written about so far. Stay tuned!

### References

[The Rust Programming Language, Chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
