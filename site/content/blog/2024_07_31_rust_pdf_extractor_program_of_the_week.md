+++
title = "rust-pdf-extractor - Turning PDF Documents into Plain Text"
description = "My Rust program of the week takes a pdf file as input and converts it to a plain text file as output."
date = "2024-07-31"
draft = false
[taxonomies]
categories = ["Projects"]
tags = ["rust", "CLI app", "file IO"]
+++

It's the last day of July 2024, just where is the time going? It's mid-week, so I'm concluding the month with my Rust program of the week.

I wrote this a couple of weeks ago, pretty much all from my own head with the barest minimum of ChatGPT suggestions. I wrote it for use at work, where I want to get pdf content into a more maleable format. There are some technical resources used in my line of work that I'd like to convert into web based content, so I thought having something that could take a pdf and convert it to plain text would be useful.

This project leverages two community crates:

- clap (for CLI input)
- pdf_extract (for doing the heavy conversion lifting)

In addition to these crates, I lean on the standard library for file IO functions.

I may eventually convert this project into an API of some sort, but this will take me awhile to figure out.

```rust
// src/main.rs

// dependencies
use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
#[arg(short, long)]
input: String,
#[arg(short, long)]
output: String,
}

// function to extract the content of the pdf and return the bytes
fn extract_content(input: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
	let content = pdf_extract::extract_text_from_mem(&input)?;
	Ok(content)
}

// function to read the input file contents
fn read_input(input_file: String, stdout: &mut dyn Write) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	writeln!(stdout, "Input file name: {}", &input_file)?;
	let content = std::fs::read(input_file)?;
	Ok(content)
}

// function to write the output file after extraction
fn write_output(output_file: String, output: String, stdout: &mut dyn Write) -> Result<(), Box<dyn std::error::Error>> {
	let path = Path::new(&output_file);
	let mut file = File::create(path)?;
	file.write_all(output.as_bytes())?;
	writeln!(stdout, "Output file name: {}", output_file)?;
	Ok(())
}

// main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	let mut stdout = io::stdout();
	let pdf = read_input(args.input, &mut stdout)?;
	let text = extract_content(pdf)?;
	write_output(args.output, text, &mut stdout)?;
	writeln!(stdout, "Conversion from pdf to plain text completed successfully.")?;
	Ok(())
}
```

I haven't put this code into a GitHub repo yet, but eventually will.
