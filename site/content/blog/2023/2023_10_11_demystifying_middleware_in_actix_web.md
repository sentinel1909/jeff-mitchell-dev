+++
title = "Demystifying Middlware in Actix Web"
description = "An article providing an overview and gentle introduction to middleware in Actix Web"
date = "2023-10-11"
[taxonomies]
categories = ["Advanced Guides"]
tags = ["rust", "web frameworks", "actix web", "middleware"]
+++

## Introduction

Earlier this year, I worked through [Zero to Production in Rust](https://www.zero2prod.com) by [Luca Palmieri](https://lpalmieri.com) using the Axum web framework. This a) forced me to learn that framework and b) force me to think when overcoming the differences.

As I moved through the book, I felt that Actix Web seemed the superior choice for someone of my skill level, as it had more hand holding. Axum, for the most part, leaves you to your own devices, which is great if you know what you're doing, but not so great if you're really just a beginner.

In late September I decided to return to Actix Web and make an API to allow me to track my daily music listening habits. I'm not going into the details of that project in this post, instead I'm going to focus on one corner that I found challenging, middleware. Per usual for me, I need to see a purpose or example, which helps me attach meaning to the abstract and enables me to understand how to use a particular language feature. Middleware in Actix Web is not well explained for newcomers. I find the docs do a decent job of explaining the high level theory, but like with the majority of Rust documentation, they lack context and simple examples.

## A Primer on Traits

Actix Web's middleware system is based on the Service and Transform traits. We can't talk about these until we brush up a little on traits in Rust. Per the Rust Book:

> A _trait_ defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use _trait bounds_ to specify that generic type can be any type that has a certain behavior.

Traits are a means of providing your own types with functionality, without having to write all that functionality from scratch yourself. They are like a contract that provide a blueprint to implement functionality.

I'm not going to go super in depth beyond this, I've linked in the references below to the in-depth chapters in The Rust Book.

## What is Middleware?

The term "middleware" identifies a type of software that sits in the middle between one thing and the next. It can be thought of as a bridge, facilitating communications between parts of a software system. Another good analogy is plumbing. Middleware connects one or more applications so that data came move through them like water through a pipe network.

## Why use Middleware?

The main advantage of middleware is it reduces complexity. In the context of a server API, middleware can combine functionality into one block, rather than each endpoint implementing its own version of that functionality. The result is reduced complexity and fewer places to introduce bugs. We'll see this in action further on in the example I've created.

Authentication is an area where middleware is often used. Authentication services can be implemented as middleware and "gatekeep" the rest of the application such that the middleware intercepts all incoming requests before passing properly authenticated users to the lands beyond. Handling authentication via middleware saves each endpoint from having to do it. The middleware authenticates the user and either lets them pass on to the endpoint they wanted, or rejects the attempt.

## How to use Middleware in Actix Web

Middleware in Actix Web centers around implementing the Service and Transform traits. _Both_ these traits must be implemented on a type. The Transform trait initializes the middleware and acts like a factory. The factory builds the particular service defined by the Service trait. One difficulty I had is separating boiler plate code from the 'real' code that you write to implement functionality. Let's take a look at the bare minimum boiler plate to get off the ground. We'll make a simple middleware to process an API key.

```rust
// dependencies
use crate::domain::appstate::AppState;
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// struct to represent the API key
#[derive(Debug, Clone)]
pub struct ApiKey;

// implement the transform trait for the ApiKey struct
impl<S, B> Transform<S, ServiceRequest> for ApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // create a new instance of the ApiKeyMiddleware struct
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyMiddleware { service }))
    }
}

// struct to represent the ApiKeyMiddleware
pub struct ApiKeyMiddleware<S> {
    service: S,
}

// implement the service trait for the ApiKeyMiddleware struct
impl<S, B> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

		// bind a call to our service as the variable 'fut'
		// we have to box a future, so that its size is known at compile time.
		// we also need to pin it in memory, because of the async operation
		let fut = self.service.call(req);
		Box::pin(async move {
			let response = fut.await?;
			Ok(response)
		})
	}
}
```

Whew! I don't know about you but I'm kind of exhausted. That's a lot of code! The trait implementations give us a lot to think about and follow. This middleware will essentially do nothing. All the hard work above just sets it up so that you can wrap your endpoints with it. As it stands above, the boiler plate will pass an unmodified request straight on to the endpoint, whatever that may be.

Alright, let's break all this down:

_Dependencies_

- we need tools from actix_web and the actix_web::dev module
  - foward_ready is a macro which, as the docs say, is an implementation of the function poll_ready which returns Ready which is a type of future that is immediately ready with a value
  - Service
    - a trait, for implementing an asynchronous operation from a Request to a Response
  - ServiceRequest
    - a service level request wrapper
  - ServiceResponse
    - a service level response wrapper
    - in the Transform trait there is a type EitherBody. EitherBody is an enum with two variants, Left and Right. Each variant can contain a BoxBody type, which is a boxed message body with boxed errors
  - Transform
    - defines the interface of a service factory and is what builds our middleware service
- we need the Error struct from actix_web

- there are a few things from the standard library that are necessary, in order to enable working with futures
  - ready
    - a function which immediately returns a future with a value
  - Ready
    - a struct which represents our future and it's associated value, it's the output of the ready function
- we'll also pull in LocalBoxFuture from the futures_util crate
  - LocalBoxFuture is a "An owned dynamically typed Future for use in cases where you can’t statically type your result or need to add some indirection"
    - it doesn't have the Send requirement, meaning it can't be transferred across thread boundaries

_Structs_

Middleware is always implemented for a type. We need two structs:

- ApiKey
  - acts a hook to trigger the middleware
- ApiKeyMiddleware
  - the actual middleware service that is built, called, and executed

_Transform trait_

When you push aside all the trait bounds and type definitions, the core of the transform trait is the new_transform function. This function takes a &self reference and a service as parameters, and returns a future. You can think of this function as a factory that creates your middleware service. It's necessary for each middleware service that you make.

_Service trait_

Similar to Transform, there are a lot of trait bounds and type definitions to wade through. Again, the core of the Service trait is the call function. This is where the meat goes. It takes a reference to self and an incoming request as parameters, and returns a future. The Future type definition constrains the call function to return a future or an error.

Clear as mud? I thought so. Let's make the boiler plate do something now.

## Meat of the Matter

This middleware will check the validity of an incoming API key. This example was created to run over on [Shuttle](https://shuttle.rs) and I won't get into the details of that, except to say the valid API key is stored in a secrets file which is read into and saved in the application state, using web::Data. This approach is overly simplistic and meant mainly for my learning purposes, so that there is something simple where I (and hopefully you) can visualize the flow. In real life you're not going to handle an API key in this fashion.

```rust
// I'll just show the completed call function forming part of the Service trait we started earlier

  fn call(&self, request: ServiceRequest) -> Self::Future {
        // get the x-api-key header from the incoming request
        let x_api_key = request
            .headers()
            .get("x-api-key")
            .and_then(|value| value.to_str().ok());

        // get the valid api key from app state
        let api_key = request
            .app_data::<Data<AppState>>()
            .map(|data| data.api_key.clone());

        // check if the api key is valid
        if x_api_key != api_key.as_deref() {
            // return a 401 unauthorized response
            let (request, _payload) = request.into_parts();
            let response = HttpResponse::Unauthorized().finish().map_into_right_body();
            return Box::pin(async move { Ok(ServiceResponse::new(request, response)) });  // error response is returned immediately, we don't need to 'await' it
        }

        // return the request if the API key is valid
        let response = self.service.call(request);
        Box::pin(async move { response.await.map(ServiceResponse::map_into_left_body) })
    }
```

Here's what's happening:

- we read the incoming API key from the "x-api-key" key/value pair that comes in with a request header, the content gets converted into a string slice
- we read in the valid API key from application state, it gets cloned to make the compiler happy...at least that's what I had to do, I'm sure someone more experience would do it in a better way
- we compare the API keys, if they don't match the request is refused and an error message is returned
- assuming the API key checks out, we pass the request through to it's appropriate endpoint.

In the API key validation if block, we split the request into it a request and payload, by destructuring into a tuple. Then, we construct the "unauthorized" error response we want to give, mapping it into the right variant of our ServiceResponse type, which is an EitherBody type because of our trait definition. The right variant of EitherBody typically holds error responses.

If the API key is valid, then we pass through the request, using the left variant of our ServiceResponse type, which represents the response from the middleware, which in this case is the unmodified request.

Ok, so, we finally have some middleware. how do we use it?

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(ApiKey).service(greet)    // our middleware "wraps" the greet endpoint
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

This is the hello world example from the Actix Web docs. I've modified to show where our middleware gets invoked as the app spins up. It's called by the .wrap() function the precedes the call to the greet endpoint handler. That's it! I haven't done it in the snip above, but you would need to bring your ApiKey type into scope with a use statement, assuming it lives in a separate module file. As a point of good practice, and given how long middleware might be, it's best to split it up into separate module files.

If you have multiple middleware pieces, you write each one separately and wrap it with a struct in the same fashion. Remember that middleware is executed in the opposite order to registration.

## Conclusion

Made it! I hope you enjoyed this short journey through Actix Web middleware. This is not exhaustive, and I'm sure there are a million nuances I've missed. I hope I've captured for you the flow of what you need to write your own. Remember, the boiler plate above can be added to your project and it will just sit and be ready for expansion. Once it's done, it's done and you just need to add your logic to the call function within the Service trait implementation.

My purpose here was to understand full blow middleware, so that's been the focus of this piece. I would be mistaken to not say that there is a simpler way to do middleware, if it's literally something dead simple. It's experimental right now, but will likely graduate sometime soon. [Simpler Actix Web Middleware](https://docs.rs/actix-web-lab/latest/actix_web_lab/middleware/fn.from_fn.html)

I invite you to dig deeper in the articles below, especially Luca's. He's developing the Pavex web framework and in doing that, having to figure out his own take a middleware system. It's a terrific read.

Now, venture forth and make some middleware! Good luck!

## Resources

[Actix Web Documentation: Middleware](https://actix.rs/docs/middleware)

[The Rust Programming Language, Chapter 10.2, Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)

[The Rust Programming Language, Chapter 19.2 Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

[Pavex DevLog #6: designing safe and ergonomic middlwares](https://www.lpalmieri.com/posts/pavex-progress-report-06/)
