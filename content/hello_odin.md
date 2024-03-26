---
title: "My First Odin Program"
date: "2024-03-25"
slug: "first-odin-program"
category: "programming languages"
tag: "odin"
summary: "An over-engineered Hello, World derivative, written in Odin"
---

# My First Odin Program

## or how to totally over-engineer a Hello, World! starter

I've decided to spend the next 100 days exploring the [Odin language](https://odin-lang.org/). As the website says, "Odin is the C alternative for the Joy of Programming". Why is the self-professed Rust shill here? That is a story for another day.

Here is my first attempt at a program in Odin:

```odin
// my first Odin program, yes, it's a HeLLo, World! derivative

// package name declaration
package main

// dependencies
import "core:fmt"
import "core:os"
import "core:strings"
import "core:testing"

// declare a constant string which forms the base of the greeting message
BEGIN_MSG :: `Hello, `

// declare a constant string which forms the end of the greeting message
END_MSG :: ` welcome to Odin!`

// hello procedure, assembles the greeting message by concatenating the elements of a string slice
// the string slice includes BEGIN_MSG, the name parameter passed via the procedure call, and the END_MSG
hello :: proc(name: string) -> string {
  greeting := [?]string{BEGIN_MSG, name, END_MSG}
  return strings.concatenate(greeting[:])
}

// the main procedure, entry point for the program, checks that the correct number of command line,
// arguments are present
// if not program operation halts with a reminder about how to use the program
// if there is a command line argument, it's assumed to be the person's name, which is passed to
// the hello() procedure to construct and return the greeting message
main :: proc() {
  if len(os.args) < 2 || len(os.args) > 2 {
    fmt.printf("Please enter your first name. [Usage: ./hello-world.exe <name>]")
    } else {
      fmt.printf(hello(os.args[1]))
  }
}

// a unit test for the hello() function happy path
@(test)
TestHelloWorks :: proc(t: ^testing.T) {
  input := "Jeff"
  testing.expect_value(t, hello(input), "Hello, Jeff welcome to Odin!")
}
```

Watch this space for more!