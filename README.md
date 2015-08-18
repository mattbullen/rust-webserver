## JSON through a Rust webserver

A side project to learn the [Rust](https://www.rust-lang.org/) language. It's a simple Heroku app built using the [Iron](http://ironframework.io/doc/iron/) framework, based on this [example](https://github.com/defyrlt/heroku-rust-cargo-hello). 

The idea is to send a query string to the server, then return a JSON object to the browser. This first version is still in progress. Right now, it lets you send a string to the server through a Node.js proxy (to get around CORS restrictions). The server formats the string then sends it back both as an iframed response page, and as a string that the browser can convert into a usable object. More complex examples are on the way, once I figure them out!

##### Try it out: http://bullen.io/rust/index.html
