# WebGPU Christmas Tree

Just a bit playing around with Rust, WASM and WebGPU.
A follow-up on [Vulkan Christmas Tree](https://github.com/jacekbilski/vulkan-christmas-tree) and [WASM Christmas Tree](https://github.com/jacekbilski/wasm-christmas-tree).
This is supposed to be, in the end, the final and ultimate version of Christmas Tree.

### Building

Make sure to install `wasm-pack` using `cargo install wasm-pack` before running `./build.sh`.

### Running

As the app is using JavaScript modules, it needs to be served by an actual HTTP server.
Simplest way is to use [Host These Things Please](https://crates.io/crates/https), so run `cargo install https`.
Now start the server in this project directory with `http` and go to [http://localhost:8000/static/](http://localhost:8000/static/).
