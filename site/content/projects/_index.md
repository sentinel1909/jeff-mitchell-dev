+++
title = "projects"
aliases = ["/projects.html"]
+++

Here are some projects I'm working on or have worked on.

_Frontend Web Dev in WebAssembly_

[NASA Imagery Viewer](https://nasa-imagery-viewer.shuttleapp.rs)

I just don't want to be the billionth person focusing on the traditional web stack of HTML/CSS/Javascript. As a hobbyist, I have the ability to explore different things. Thii project is a simple web page, compiled to WebAssembly using [Yew](https://yew.rs) for Rust, served up with a minimal [Rocket](https://rocket.rs) web server, hosted on [Shuttle](https://shuttle.rs).

_Backend Development - APIs_

[Crusty Rustacean Newsletter API](https://github.com/sentinel1909/crusty-rustacean-api.git)

This project is my completed take on [Zero to Production in Rust](https://www.zero2prod.com) by [Luca Palmieri](https://www.lpalmieri.com). I did it using the Axum web framework, instead of Actix Web. I deployed this project, for a time, on Railway, but took it down as I became increasingly nervous about having something less than amateurish exposed to the internet. I'm thinking about getting it going again...on Shuttle.

_Doing Things with Community Crates_

[Fun with Nom](https://github.com/sentinel1909/fun-with-nom.git)

Early in 2024, I developed a wierd obsession with the Nom crate. I tried to using late in 2023 while working through Shuttle's Christas Code Hunt. In typical fashion for me, I was way over my head, running before even crawling. I figured I'd start building out an API that returned the word count of a sentence handed to it. The project is not deployed, but I may yet return to it. There are precious few resources out there for mere mortals on how to use this crate.

_Fullstack Rust_

[Rivet Head Diary](https://rivet-head-diary.shuttleapp.rs)

In the "bitten off more than I can chew category", enter the Rivet Head Diary. This is a full stack Rust app, Yew on the front and Axum on the back, which will eventually be a way for me to blog about my music listening habits. This thing is becoming a monster that I may never finish.

I intend on writing a CLI companion app as part of this project, in addition to having a web form that's protected by JWT authentication, so that I can log in from anywhere and add new entries.

Don't go to the site just yet, it doesn't do much.
