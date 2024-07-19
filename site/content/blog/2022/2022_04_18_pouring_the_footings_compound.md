+++
title = "Pouring the Footings: Compound Data Types"
description = "A continued study of the basics in Rust, focusing on compound data types."
date = "2022-04-18"
[taxonomies]
categories = ["Beginner Guides"]
tags = ["rust", "data types", "compound"]
+++

Selecting a data structure model is a key step in software development. Previously, I showed the very basic single data types in Rust. This article covers compound data types. Compound types can incorporate multiple building blocks in one overall type.

### Tuples

A tuple can contain multiple values, each with their own type, into one single entity. Tuples have a fixed length, they cannot grow or shrink later on. Tuples are stored on the stack in memory. Care is needed up front with creation because a tuples size can’t be modified later. You can modify the values inside the tuple, but new values can’t be added to it.

We can declare a tuple like so:

```
let survey_result: (&str, u8, bool) = ("John", 36, true);
```

We’ve created a variable name called survey_result and bound a string slice (haven’t talked about strings yet, don’t worry for now), an integer, and a boolean together into a tuple. This variable could represent the data received after submission of a survey form. There are type annotations immediately following the variable name. These are optional. In most cases, Rust can “infer” (figure out) the types needed.

There are a couple of different ways to access the values inside a tuple.

_Via Indicies_

Individual values inside a tuple are indexed from 0, 1, 2… similar to arrays or vectors. Values can be accessed with a (.) followed by the value index, like so:

```
fn main() {
    let survey_result: (&str, u8, bool) = ("John", 36, true);
    let name = survey_result.0;
    let age = survey_result.1;
    let answer = survey_result.2;
}
```

_Via Destructuring_

The index method is a little verbose and it can be difficult to track the indicies if the tuple is large. It’s more elegant (and efficient) to use the destructuring method, like so:

```
fn main() {
     let survey_result: (&str, u8, bool) = ("John", 36, true);
     let (name, age, answer) = survey_result;
     println!("{}, aged {}, answered {} on the survey form.", name,    age, answer);
```

We’ve created the same tuple named survey_result and bound the same three values as the previous example. Next, we bind the individual values within the tuple to three new variables named name, age, and answer. We’ve used the destructuring pattern instead of three separate let statements. Finally, we print out the result to the console.

Tuples are useful for small data structures, but they become unwieldy for more complex needs. The better choice is a struct, which will be the topic of a future article.

### Arrays

The next compound data type is an array. Similar to tuples, arrays are fixed length and cannot grow or shrink. Unlike tuples, arrays can only contain the same type of basic values. Arrays are useful in situations where a fixed number of elements is desired. An example array of the names of the days of the week can be created like so:

```
let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
```

I haven’t discussed memory yet as it relates to Rust, but similar to tuples, arrays are held in memory on the stack. This is because they are a known, fixed length, similar to the basic data types. Rust knows how much memory to allocate for them at compile time.

Arrays are declared with square brackets and can receive an optional type declaration, like so:

```
let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
```

We’ve created an array of six integers and bound it to the variable named numbers. The type declaration is enclosed in square brackets and contains the type followed by a semicolon and the number of elements in the array.

If for some reason an array is needed that initially has all the same elements, a short form can be used to declare it, like so:

```
let initial_values = [3; 3];
```

This creates an array with three elements, all having the integer value 3.

Elements within an array are accessed by referencing the variable name and appending an index surrounded with square brackets, like so:

```
fn main() {
     let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]

     let midweek = weekdays[2];
     let endofweek = weekdays[4];
}
```

Here we declare an array variable that contains the five days of the week. Individual elements are indexed from 0 to 4. We then assign Wednesday to a new variable called midweek and Friday to a new variable called endofweek.

It’s important to remember the 0 start index of an array. I messed up while writing this 🙂 I think whoever decided to start indexing at zero should have their head examined, but it is what it is.

To conclude the discussion on arrays, Rust will help you when accessing array elements by automatically checking that the specified index is less than the array’s length. A runtime error will be received in the event the program tries to access an array index that is beyond the end of the array.

### Conclusion

This article has covered Rust’s compound data types, tuples and arrays. I haven’t seen or written giant amounts of Rust code yet, but I’ve not seen these data structures used particularly often. The go-to seems to be vectors, particularly vectors of structs. The potential unwieldiness of tuples and the inability of arrays to grow or shrink I think limits their use cases. They both have the advantage of being stored on the stack in memory, so they are cheap to make and quick to access. At the end of the day, both are an available tool and you the programmer must choose what works best for your particular problem.

References

[The Rust Programming Language, Chapter 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)
