+++
title = "markdown-html-rs - Converting Markdown Content to HTML"
description = "Highlighting markdown-to-html, a Rust CLI program which helps you convert markdown content to HTML."
date = "2024-07-24"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Projects"]
tags = ["rust", "CLI app", "markdown"]
+++

Alright folks, here it is...your Rust program of the week. I call it "markdown-html-rs". It takes in a file with markdown content, including frontmatter, and outputs said markdown content to html.

This project leverages the following community crates:

- clap (for CLI input)
- gray-matter (for parsing YAML style front matter)
- regex (to help identify front matter)
- thiserror (to reduce boilerplate in error handling)
- serde (for deserializing the incoming data)

```rust
// main.rs
// A command line program which takes a markdown file as input, converts to HTML, and outputs the HTML file

// dependencies
use clap::Parser;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::string::FromUtf8Error;
use thiserror::Error;

// enum to represent error types
#[derive(Error, Debug)]
enum ConversionError {
    #[error("File read error: {0}")]
    FileRead(std::io::Error),
    #[error("Deserialization error: {0}")]
    Deserialization(serde_json::error::Error),
    #[error("File write error: {0}")]
    FileWrite(std::io::Error),
    #[error("HTML write error: {0}")]
    HTMLWrite(std::io::Error),
    #[error("Markdown conversion error: {0}")]
    MarkdownConversion(FromUtf8Error),
    #[error("Regex error: {0}")]
    Regex(regex::Error),
}

// struct to represent command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

// struct to represent the front matter of the markdown document
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    date: String,
    tags: Vec<String>,
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: "".to_string(),
            date: "".to_string(),
            tags: Vec::new(),
        }
    }
}

fn main() -> Result<(), ConversionError> {
    // create an output buffer
    let mut stdout = io::stdout();

    // get the file name from the command line input
    let args = Args::parse();

    // read the file contents and save it as a vector of u8
    // convert the file contents into a markdown string
    let file_contents = fs::read(args.filename).map_err(ConversionError::FileRead)?;
    let markdown_input =
        String::from_utf8(file_contents).map_err(ConversionError::MarkdownConversion)?;

    // parse the front matter in the input string and deserialize it into a FrontMatter struct
    // remove the front matter, leaving on the body content of the markdown file
    let matter = Matter::<YAML>::new().parse(&markdown_input);
    let front_matter: FrontMatter = matter
        .data
        .as_ref()
        .map(|data| data.deserialize())
        .transpose()
        .map_err(ConversionError::Deserialization)?
        .unwrap_or_default();

    writeln!(stdout, "{:?}", front_matter).map_err(ConversionError::FileWrite)?;
    let frontmatter_regex =
        Regex::new(r"---\s*\n(?s:.+?)\n---\s*\n").map_err(ConversionError::Regex)?;
    let markdown_body = frontmatter_regex.replace(&markdown_input, "");

    // parse the markdown body and convert it to html, any html tags in the markdown file are passed through
    let parser = pulldown_cmark::Parser::new(&markdown_body);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // write the html output file
    fs::write("output.html", html_output).map_err(ConversionError::HTMLWrite)?;
    writeln!(stdout, "Markdown converted and saved to output.html")
        .map_err(ConversionError::FileWrite)?;

    Ok(())
}
```

The GitHub repo lives [here](https://github.com/sentinel1909/markdown-html-rs).
