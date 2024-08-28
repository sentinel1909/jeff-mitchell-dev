+++
title = "Solid HTML: A Starting Foundation"
description = "An article which provides some starting HTML for your web endeavours"
date = "2024-08-27"
authors = ["Jeff Mitchell"]
draft = true
[taxonomies]
categories = ["Vanilla Web"]
tags = ["html"]
+++

I get down on myself, a lot. I constantly compare myself to others and their journey in coding. I'm not (yet) banging up against the rough edges of Rust, solving the rocket science problems necessary to driving the language foward.

I need to re-frame the narrative.

I need to quit with the negative self-judgment.

My journey is my journey, and all that matters is, day by day, month by month, year by year, I'm steadily able to do more.

Alright, enough navel gazing, this post wasn't really intended to be about my negative self image. I wanted to continue for a bit on the theme of basics.

Like it or not, HTML is the starting point of the web, even in 2024.

The first thing the browser loads is...yep, HTML.

Earlier this year, I made a [vanilla web template](https://github.com/sentinel1909/vanilla-web-template) starter for myself. Here is the `index.html`:

```html
<!-- index.html -->

<!-- This is an HTMl page which describes structure of the web site content -->
<!-- The head tag links in the page styles, contained in screen.css, the media tag -->
<!-- helps the user agent (maybe not a browser) understand that the styles are for a screen -->
<!-- The body tag contains the page content, and uses the semantic HTML tags <header>, <main, and -->
<!-- <footer> to describe the header, main content, and footer areas of the page -->
<!-- The scripts tag links in the supporting JavaScript file, which fills in the current year in the footer. -->

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <!-- set up some preloaders, so the browser knows about them as soon as possible -->
    <link rel="preload" href="styles/screen.css" as="style" />
    <link rel="preload" href="scripts/scripts.js" as="script" />
    <title>Vanilla Web Template | Home</title>
    <link rel="stylesheet" type="text/css" href="screen.css" media="screen" />
    <link rel="icon" type="image/x-icon" href="static/favicon.ico" />
  </head>
  <body>
    <header>
      <h1>Hello, World!</h1>
      <h2>Welcome to the vanilla web.</h2>
    </header>
    <main>
      <section>
        <article>
          <p>
            This is a plain vanilla web site, written with good 'ol
            HTML/CSS/JavaScript.
          </p>
        </article>
      </section>
    </main>
    <footer>
      <p>&copy; <span id="year"></span> Jeffery D. Mitchell</p>
    </footer>
    <script src="scripts/scripts.js"></script>
  </body>
</html>
```

I'm a big believer in semantic web, so my starter is reflective of that and will get you off on the right foot. I try to keep `<div>` soup to a minimum. From this minimal starter, you can build out a whole site by adding sub-pages, links, and more content.

Here is the repo for the rest of my [vanilla web template](https://github.com/sentinel1909/vanilla-web-template).

I hope it's a useful starting point for you, if you don't already have your own.
