+++
title = "Decisions Decisions"
description = "An introduction to the decision making tools and syntax in Rust."
date = "2022-04-27"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "decision making"]
+++

Software capability would be very limited if there were no ability to make decisions and follow different logic paths. The ability to branch and provide different outcomes, based on either internal results or external user input, is essential. Rust provides basic decision logic in the form of the if and else if expressions.

### The if Expression

An if expression starts with the if keyword and is followed by a condition. Immediately after the condition is a block of code, surrounded by curly braces, which is executed if the condition proves true. If the condition is not true, the code after the condition is skipped entirely and execution flows to the next statement.

Let’s see an example:

```
fn main() {
     let year = 1984;
     if year > 2000 {
     println!(“The year is: {}”, year);
     }
}
```

We first declare a variable named year and bind to it the integer 1984. Next, we start the if expression. The condition checks that the value bound to the year variable is greater than 2000. In this instance, it is not, so the code in the curly braces is ignored and the program ends. Nothing is output.

If the condition was true, then the code within the curly braces would execute and we’d see the message “The years is: 1984” output to the console.

Conditions can be chained with operators like or || and &&. This provides the ability to make decisions with more complex criteria.

```
fn main() {
     let year = 2022;
     if year > 2000 && year < 2025 {
     println!(“The year is: {}”, year);
     }
}
```

The condition in this code will evaluate true if the year is greater than 2000 and less than 2025. Our year variable has a value of 2022, which satisfies both conditions. The code block after the condition will execute and the message “The year is: 2022” will output to the console.

When the condition after an if statement is not satisfied, code execution jumps to whatever comes after the if expression. An optional else expression can be added to provide another pathway for code to follow. The code preceding the else statement will execute only when the condition evaluates to false.

```
fn main() {
    let number = 26;
     if number > 30 {
     println!(“{} is greater than 30”, number);
     } else {
          println!(“{} is less than 30”, number);
     }
}
```

Here, we declare a variable named number and bind to it the value 26. We check if the value assigned to number is greater than 30. In this case, it is not, so execution transfers to the else expression and the code within the curly brackets preceding it is executed. The message “26 is less than 30” is printed to the console.

If the variable year contained a value that was greater than 30, the code preceding the condition is executed instead, resulting in a different message being output to the console. In this case, the else expression would be skipped and code flow would proceed to whatever comes after.

### The else…if Expression

If there is a need to check multiple conditions, the else if expression is available. Let’s take the example given in Chapter 3.5 of The Rust Programming Language, as it illustrates an important nuance.

```
fn main() {
     let number = 6;
     if number % 4 == 0 {
          println!(“number is divisible by 4”);
     } else if number % 3 == 0 {
          println!(“number is divisible by 3”);
     } else if number % 2 == 0 {
          println!(“number is divisible by 2”);
     } else {
          println!(“number is not divisible by 4, 3, or 2”);
     }
}
```

We bind the integer 6 to the variable named number. We then have a list of conditions to check. Rust will execute the first block of code for which a condition is satisfied, which in this case is the first else expression. The number six is evenly divisible by 3, so the message “number is divisible by 3” is output to the console. Even though some of the other conditions in the series may be true, it doesn’t matter. Only the first true one is executed, all the other possibilities are ignored.

Nested if else if expressions can get out of hand very quickly resulting in messy code that’s hard to reason through. Rust has a more advanced option, the match statement, which is very powerful. I’ll explore it in future writings.

### Using if in a let Statement

Given if is an expression, it can form part of a let statement and be used to assign variables. Again, an example from The Rust Programming language is useful.

```
fn main() {
     let condition = true;
     let number = if condition { 5 } else { 6 };
     println!(“The value of number is: {}”, number);
}
```

Here, we bind the boolean value true to a variable named condition. Next, the integer 5 is bound to the variable number. This happens because the variable condition is true. An if statement, fundamentally, is checking whether something is true, then executing code after. The else part of the expression is ignored. If the condition was false instead of true, the integer 6 would be bound to number.

### Conclusion

I’ve shown you Rust’s most basic tools for coding decision-making. The combination of if, else, and else if provide the ability for different code paths to execute, thus varying the output as needed based on internal conditions or a combination of internal conditions and external input.

References

[The Rust Programming Language, Chapter 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
