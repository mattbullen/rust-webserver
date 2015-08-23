## JSON via a Rust webserver and a Node.js proxy

A side project to learn the [Rust](https://www.rust-lang.org/) language. It's a simple Heroku app built using the [Iron](http://ironframework.io/doc/iron/) framework, based on this [example](https://github.com/defyrlt/heroku-rust-cargo-hello). 

The idea is to send a query string to the server to retrieve some text content from a file, then return that content to the browser as a JSON-formatted string. This version also uses a Node.js proxy to work around Heroku/CORS restrictions, since the front end is hosted on a different domain than the app.

##### Try it out: http://bullen.io/rust/index.html
