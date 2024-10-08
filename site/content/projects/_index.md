+++
title = "projects"
aliases = ["/projects.html"]
+++

Here are some projects I'm working on or have worked on.

_Frontend Web Dev in WebAssembly_

[Astronomy Lover](https://astronomy-lover.net)

I just don't want to be the billionth person focusing on the traditional web stack of HTML/CSS/Javascript. As a hobbyist, I have the ability to explore different things. Thii project is a simple web page, compiled to WebAssembly using [Yew](https://yew.rs) for Rust, served up with a minimal [Rocket](https://rocket.rs) web server, hosted on [Shuttle](https://shuttle.rs). It fetches the daily photo from the NASA Astronomy Photo of the Day API and displays it. I'd like to eventually embelish this project with a history feature, so that you can look back at past images or videos.

_Backend Development - APIs_

[Crusty Rustacean Newsletter API](https://github.com/sentinel1909/crusty-rustacean-api.git)

This project is my completed take on [Zero to Production in Rust](https://www.zero2prod.com) by [Luca Palmieri](https://www.lpalmieri.com). I did it using the Axum web framework, instead of Actix Web. I deployed this project, for a time, on Railway, but took it down as I became increasingly nervous about having something less than amateurish exposed to the internet. I'm thinking about getting it going again...on Shuttle.

[Shuttle Template - Hyper](https://github.com/sentinel1909/shuttle-hyper-emplate.git)

I mess about with stuff so that you don't have to. Anyone in their right mind would build their API with Axum (or the other framework of your choice) and call it a day, but not me! I'm going nap-of-earth with Hyper and Tokio, just in case you want to see for yourself what not to do. I've managed to create a skeleton API and have turned it into a template of sorts. It's a mainly a learning ground for me, as I can't stand just reading theory without getting my hands dirty.

_Frontend Development_

[Shuttle Template - Yew](https://github.com/sentinel1909/shuttle-template-yew)

I don't want to be the billionth person learning React. I enjoy front end development with Rust and have been steadily picking up how to do things with [Yew](https://yew.rs).

_Doing Things with Community Crates_

[Fun with Nom](https://github.com/sentinel1909/fun-with-nom.git)

Early in 2024, I developed a wierd obsession with the Nom crate. I tried to using late in 2023 while working through Shuttle's Christas Code Hunt. In typical fashion for me, I was way over my head, running before even crawling. I figured I'd start building out an API that returned the word count of a sentence handed to it. The project is not deployed, but I may yet return to it. There are precious few resources out there for mere mortals on how to use this crate.
