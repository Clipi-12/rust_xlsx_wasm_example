# Limitations
* This is supposed to be a minimal example of using rust_xlsxwriter, but it is
quite a large project to be considered minimal.
This will be difficult to fix, though, since most files are present to replace
unimplemented-std-methods with browser/javascript-vm functions.
* Node and Deno crash javascript-side whenever an io error occurs (instead of
returning a Result that we could catch in Rust).

# Setup

This repo uses [cargo-make](https://sagiegurari.github.io/cargo-make/) to
manage all the different compilation targets. It is by no means
neccessary to create a WASM application, but using it reduces complexity, since
most targets are built by repeating similar commands.

All rules can be found in [Makefile.toml](Makefile.toml), and each contains a
profile called _release_ that selects the rustc compilation flag and calls
`wasm-opt` when enabled.\
Remember that profiles in cargo-make must be selected
**before** the rule\
(e.g. `cargo make -p release all` instead of ~~`cargo make all -p release`~~).

```bash
cargo install cargo-make
```

### Dependencies needed to build against a browser, Node.js, or Deno

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli wasm-opt
```

> [!WARNING]
> The `wasm-bindgen` command may fail with a linkage error if the
> _wasm-bindgen-cli_ cargo-installed-package doesn't match the _wasm-bindgen_
> cargo-added-package. If this happens to you, just `cargo update` or read the
> error thrown in stderr.

### Dependencies needed to build against WASI

```bash
rustup target add wasm32-wasip1
```

You can also use the experimental
[_wasm32-wasip1-threads_](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html)
target, or the [stabilized](https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html)-yet-experimental
[_wasm32-wasip2_](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html)
target. Both of them implement the std fully, meaning they can use
`std::thread::spawn`, which _wasm32-wasip1_ can't.\
If you end up choosing one of the above instead of _wasm32-wasip1_, change the
variable `WASM_WASI` in [Makefile.toml](Makefile.toml) accordingly. Also, since
they are experimental, you may need to use nightly instead of stable, so change
the value in [rust-toolchain.toml](rust-toolchain.toml) accordingly.

```bash
rustup target add wasm32-wasip1-threads
rustup target add wasm32-wasip2
```

# Building and Running

### Running in browser

1. `cargo make browser`
1. Open a localhost server in **pkg/browser**.
For example, with `npm i -g http-server` then\
`http-server -c 1 -p 8080 pkg/browser`
1. Open a browser and go to <http://localhost:8080/index.html>

### Running in Node.js

```bash
cargo make nodejs
node pkg/nodejs/rust_xlsx_wasm_example.js
```

### Running in deno

```bash
cargo make deno
deno run --allow-write --allow-read pkg/deno/rust_xlsx_wasm_example.js
```

### Running _natively_ in WASI

In order to **run** the .wasm file, you will need to have
[Wasmtime](https://wasmtime.dev/) installed.\
`--dir=.` allows WASM to _preopen_ the current directory (essentially giving it
access to it, which it normally can't since WASM is sandboxed).

##### wasm32-wasip1 target triple

```bash
cargo make wasi
wasmtime --dir=. pkg/wasi/rust_xlsx_wasm_example.wasm
```

##### wasm32-wasip1-threads target triple

```bash
cargo make wasi
wasmtime --wasi threads --dir=. pkg/wasi/rust_xlsx_wasm_example.wasm
```

# Utilities

To build all targets:

```bash
cargo make all
```

To run both `cargo clean` and `rm -rf pkg` (but platform independent):

```bash
cargo make clean
```
