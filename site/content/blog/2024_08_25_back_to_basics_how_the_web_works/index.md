+++
title = "Back to Basics: How the Web Works"
description = "An overview of how the web gives you content."
date = "2024-08-25"
authors = ["Jeff Mitchell"]
draft = false
[taxonomies]
categories = ["Back to Basics"]
tags = ["web", "html", "css", "javascript"]
+++

## Back to Basics

Carrying on from [yesterday](@/blog/2024_08_25_back_to_basics_how_the_web_works/index.md), I'm going to write a few pieces about the basics of how the web works. Much of my motivation for getting into Rust was WebAssembly, and wanting to take a different path to become a productive web developer. I haven't spent near enough time just understanding how exactly the web functions. I've had a stubborn refusal to really dig into the basic building blocks. I'll never be independent if I don't understand these foundations.

So, I'm going to rectify that.

## The World Wide Web

> "We've learned Earth's languages through the World Wide Web" - Optimus Prime

So what is the world web web anyway? Does anyone even call it that anymore?

Imagine a city in which there are buildings that offer services. Every building has an address and when you visit there as a client, there are helpful staff who give back information which you can then take and assemble into something meaningful. The buildings are servers and they offer web sites. You, as client act as the web browser. When you visit an internet location, a server answers and gives you back a series of files which your browser takes and renders into something meaningful.

This is overly simplistic, but there's a lot to unpack at each step and having a high level view of what's going on can help.

Every web site, every last one, is built on this foundation of technologies:

- HTML
- CSS
- JavaScript

The methods and techniques may vary, but at the end of the day that's what the browser needs to render information to you as the viewer.

A web server gives back an HTML file (usually called index.html) which has links to styles (CSS) and scripts (JavaScript). When a browser makes requests for these resources, it does that via links available in the HTML file.

## Structure (Hyper-Text Markup Language, HTML)

Every web site needs bones and a skeleton. It's HTML's job to express how a web site, including its content and structure, is to be represented. This is the first thing the browser loads. The browser constructs a model in memory, called the Document Object Model (DOM). I always feel slightly dirty when I say DOM...I digress...

## Styles (Cascading Style Sheets, CSS)

It's the job of CSS to express what a web site looks like. CSS rules allow you to selectively target HTML elements and apply a style to them. This is the second thing loaded by the browser. The browser also constructs a CSS Object Model (CSSOM) which can be targeted and manipulated dynamically with JavaScript. The style and structure are ultimately combined into a "render tree" which the browser uses to "paint" the final web site into view.

## Functionality (JavaScript)

A web site can exist with only HTML and CSS, technically you don't need anything else. However, it's static and relatively boring. You can do a lot with these technologies now, more than 10 years ago, and depending on what you're doing minimal interactivity might be fine. Generally speaking though, you're going to need some JavaScript. It enables interactivity, the ability to manipulate state, and the ability to change the page content, structure, and style dynamically. JavaScript is the very last thing the browser loads.

## Closing

This has been a very basic overview of the fundamental web technologies and how individual web sites work. I'll explore each one further and write about them over the next while. You can also read the resources I've linked below for more background!

## Resources

- [How the web works](https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/How_the_Web_works)

- [Constructing the Object Model](https://web.dev/articles/critical-rendering-path/constructing-the-object-model)
- [Render-tree Construction, Layout and Paint](https://web.dev/articles/critical-rendering-path/render-tree-construction)
