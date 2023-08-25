# README WIP

# Issues / TODOS
* This is supposed to be a minimal example of using rust_xlsxwriter, but it is quite a large project to be considered minimal. This will be difficult to fix, though, since most files are present to replace unimplemented-std-methods with browser/javascript-vm functions
* Node and Deno crash javascript-side whenever an io error occurs (instead of returning a Result that we could catch in Rust)


# Setup
> cargo install cargo-make

Dependencies needed to build against a browser, Nodejs, or Deno:
> rustup target add wasm32-unknown-unknown

> cargo install wasm-bindgen-cli wasm-opt

Dependencies needed to build against WASI:
> rustup target add wasm32-wasi

# Building and Running

### Running in browser
> cargo make [-p release] browser

> `Open a localhost server in pkg/browser. For example, with`  
> npm i -g http-server  
> http-server -c 1 -p 80 pkg/browser

> Open a browser and go to localhost/index.html

### Running in nodejs
> cargo make [-p release] nodejs

> node pkg/nodejs/rust_xlsx_wasm_example.js

### Running in deno
> cargo make [-p release] deno

> deno run --allow-write --allow-read pkg/deno/rust_xlsx_wasm_example.js

### Running ___natively___ in WASI
> cargo make [-p release] wasi

In order to __run__ the .wasm file, you will need to have [Wasmtime](https://wasmtime.dev/) installed
> wasmtime --dir=. pkg/wasi/rust_xlsx_wasm_example.wasm

`--dir=.` allows WASM to _preopen_ the current directory (essentially giving it access to it, which it normally can't since WASM is sandboxed) 

# Utilities
To build all targets:
> cargo make [-p release] all

To run both `cargo clean` `rm -rf pkg` (platform unspecific):
> cargo make clean
