+++
title = "Bigger Building Blocks: Structs Reloaded"
description = "A continuation of the discussion on structs, bringing in advanced concepts."
date = "2022-06-11"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "structs"]
+++

In a previous article, I introduced you to structs in Rust. Structs are flexible data structures that allow the grouping of fields together under one roof. Structs have a lot of capability. Let’s take a look at the things I didn’t cover last time out.

### The Field Init Shorthand

The fields of a struct can be initialized quickly by using the field init shorthand. Let’s see this through an example.

We’ll start by defining a struct to hold information about photos. Our struct will contain fields for an identifier, the date the photo was taken, the time the photo was taken, and the location the photo was taken:

```rust
struct Photo {
	id: i32,
	date: String,
	time: String,
	location: String,
}
```

Next, we build a function that’s used to create and return our Photo struct with information in its fields:

```rust
fn build_photo(id: i32, date: String) -> Photo {
	Photo {
		id,
		date,
		time: "1.00pm".to_owned(),
		location: "Boundary Bay, Vancouver, BC".to_owned(),
	}
}
```

This function fills in the id and date fields of the Photo struct by using the id and date parameters received from the function. Because the struct fields have the same names as the incoming parameters of the function, they are automatically populated. We choose to initialize the other two fields manually, but there’s no reason why the function couldn’t have as many parameters as the struct fields. In this case, all of them would be automatically initialized.

### The Update Syntax

It’s frequently useful to create an instance of a struct by copying data from another one. There’s a nice shorthand to achieve this, called the struct update syntax.

```
fn main() {
	let photo2 = Photo {
		date: "2022 06 05".to_owned(),
		..photo1
	};
}
```

In this snip, we’re declaring a new variable called photo2, and then initializing it with all the fields from a previously declared photo1 variable, but changing the date field to have new data.

### Tuple Structs without Named Fields

Structs don’t necessarily need their fields spelled out. We can combine them with the tuple data structure. Let’s say we need to represent a position in space. This can be done with three coordinates, typically noted as x, y, and z.

```rust
struct Coordinates(i32, i32, i32)
```

Here, we declare a tuple struct that contains three integers to represent our x, y, and z coordinates. This struct can then be used to contain position data, like so:

```
fn main() {
	let position1 = Coordinates(0, 1, 5);
}
```

The data inside tuple structs can be accessed using dot notation and the positions of the particular fields. Our Coordinates struct has three fields, so you could do this to access the individual data points:

```rust
let x = position1.0;
let y = position1.1;
let z = position1.2;
```

Tuple structs are best for containing small quantities of fields where it’s easy to remember their purpose. If you need more than say two or three fields, it’s probably better to use a struct with named fields.

Tuple structs form their own type, even though the fields within two tuple structs might be the same. You can’t mix and match two different tuple structs together.

Unit Structs with No Fields
Structs do not need to contain data. They can be made into “unit structs”. Rust has a concept known as the “unit type”, which is denoted by round brackets with nothing inside, (). Let’s take a look at the example used in the Rust Book:

```rust
struct AlwaysEqual;

fn main() {
	let subject = AlwaysEqual;
}
```

We declare our struct using the name AlwaysEqual. There’s no need to explicitly write empty brackets, we can just terminate our declaration with a semicolon. Then, in our main function, we declare an instance of our AlwaysEqual type by binding it to a variable named subject.

Structs created in this manner are most useful when creating Traits, which will come in a future article. Traits are a way of defining common behaviour and are a powerful piece of the Rust puzzle.

### Conclusion

To recap, in this article I’ve shown you the remaining concepts that make structs an important part of the Rust language. I’ve covered shortcuts for initializing data and updating the information within a struct. I’ve also covered the notion of tuple structs, useful for small amounts of data where named fields aren’t a necessity. Lastly, I touched on the notion of unit structs that have no data but instead can be used to represent a type.

### References

[Chapter 5.2, Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
