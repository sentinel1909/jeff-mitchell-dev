// src/lib/utilities.rs

// dependencies
use gray_matter::engine::YAML;
use gray_matter::Matter;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

// enum to represent error types
#[derive(Error, Debug)]
pub enum ConversionError {
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

// struct to represent the front matter of the markdown content
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FrontMatter {
    title: String,
    date: String,
    tags: Vec<String>,
}

// implement the default trait for the FrontMatter struct
impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: "".to_string(),
            date: "".to_string(),
            tags: Vec::new(),
        }
    }
}

// function to convert raw markdown files to HTML and separate frontmatter
pub fn convert_to_html() -> Result<(), ConversionError> {
    let mut stdout = io::stdout();

    let path = PathBuf::from(r"content/markdown/test.md");
    let file_contents = fs::read(path).map_err(ConversionError::FileRead)?;

    let markdown_input =
        String::from_utf8(file_contents).map_err(ConversionError::MarkdownConversion)?;

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

    let parser = pulldown_cmark::Parser::new(&markdown_body);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    fs::write("content/html/output.html", html_output).map_err(ConversionError::HTMLWrite)?;

    Ok(())
}
