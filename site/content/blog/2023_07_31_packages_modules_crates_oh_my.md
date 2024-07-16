+++
title = "Packages, Modules, and Crates..Oh My!"
description = "An overview of to structure a Rust program, using crates, packages, and modules."
date = "2023-07-31"
[taxonomies]
categories = ["Intermediate Guides"]
tags = ["rust", "organization"]
+++

Today, I will give you my take on crates, packages and modules. I've found the organization system for Rust programs to be difficult to master. However, time and practice are solidifying things for me.

Let's dive in...

## Crates

A crate is the smallest possible Rust program. The Rust compiler considers this code sample to be a crate:

```Rust
fn main() {
  println!("I am a crate.");
}
```

An important concept is the crate root. The root is the place the compiler starts from when building out a program. In the tiny example above, whatever we name the file, say "something.rs", it is the crate root.

There are two types of crates, _binary_ crates and _library_ cates. A _binary_ crate is self contained in that it has an executable file, containing a main function, which can be run on its own without any further action. A _library_ crate contains some piece of functionality meant to be depended on by other crates. It doesn't have a main function and can't run on its own.

A common way to organize a Rust program is to split it into a binary crate and a library crate. The binary crate contains the executable file, named main.rs or something else you choose. The binary crate then pulls in added functionality from its library crate.

## Packages

A package is a bundle of one or more crates that combine to provide some piece of functionality. A package contains a _cargo.toml_ file which explains to the Rust compiler how to build the crates contained within the package. Packages can contain one or more binary crates, but only one library crate. There must be at least one crate in the package, be it binary or library.

The presence of a Cargo.toml file at the root of the project directory defines the package. By default, Cargo follows the convention that src/main.rs is the root of a binary crate and src/lib.rs is the root of a library crate. The name of the package is taken from the name of the binary or the name of the library, whichever one is present. You can adjust this if you wish and I'll talk about it in the next issue.

It's totally up to you as to how to organize your Rust programs. I've found a useful way to organize is to have one binary crate, containing a main.rs file which has initialization and startup logic. The binary crate then calls into the library crate which contains the meat of the program. If your Rust program is small, then it's overkill to organize in this way. However, for larger code bases, such as APIs, organizing like this helps keep the code modular and readable.

## Modules

Crates can be further divided with modules which can live either in one file or separate files. Modules serve as an organizational tool, allowing code to be grouped into units for easier re-use. A second purpose of modules is to control privacy of items. Code in a module is private by default, meaning it's not visible to other code outside that module. Modules can be made public, meaning they are available to be made into dependencies within the program as a whole. You can put modules all into one file, but this can quickly become difficult to navigate. It's usually preferable to setup a structure in which modules are organized into separate files.

## Paths

The Rust compiler needs to know where things are. A path can be used in a similar fashion to navigating file system on Windows, Linux, or macOS. Paths can take a couple of forms:

- absolute: the full path starting from the crate root (begins with the crate name for an external crate and begins with the literal crate for code from the current crate)
- relative: starts from the current module and uses the keywords self, super or an identifier in the current module (super, for example, will reference the next level up)

## Use Keyword

The use keyword brings a module into scope, allowing its contents to be accessible to other parts of a Rust program.

## Namespace Operator

The namespace operator in Rust is a double colon, :: The namespace operator is used in conjunction with the use keyword.

Putting it all together...

With all these concepts in mind, how do we put it all together? Stay tuned...in a future post I'll show you how to start using these concepts to make your Rust programs more understandable.
