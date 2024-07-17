+++
title = "nf - An Empty File Maker"
description = "Highlighting nf, a Rust program which makes an empty, new file."
date = "2024-07-16"
[taxonomies]
categories = ["Projects"]
tags = ["rust", "CLI app", "file IO"]
+++

I'm going to try to start doing "program of the week" types of articles. Here is the first. It takes a command line argument (the desired file name) and makes that into an empty file.

It uses the following community crates:

- command line argument parsing: [clap](https://crates.io/crates/clap)
- colourful, consistent error messages: [color-eyre](https://crates.io/crates/color-eyre)

```rust
// nf main.rs

// dependencies
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{self, Write};

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

// main function, program entry point
fn main() -> Result<()> {
    // initialize color_eyre
    color_eyre::install()?;

    // get the command line arguments
    let args = Args::parse();

    // create a handle for writing output
    let mut stdout = io::stdout();

    // create the new file
    let new_file = File::create(args.file);

    // write a success message or an error if the file couldn't be created
    match new_file {
        Ok(file) => write!(stdout, "{:#?} successfully created", file)?,
        Err(err) => write!(stdout, "{:#?}", err)?,
    }

    Ok(())
}
```

The GitHub repo for this project is here: [nf](https://github.com/sentinel1909/nf.git)
