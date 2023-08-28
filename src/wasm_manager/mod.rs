#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
))]
mod javascript;
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
))]
pub use javascript::*;
#[cfg(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
))]
#[cfg(any(
    all(feature = "browser", feature = "node"),
    all(feature = "browser", feature = "deno"),
    all(feature = "nodejs", feature = "deno"),
    not(any(feature = "browser", feature = "nodejs", feature = "deno")),
))]
compile_error!("Feature \"browser\", \"nodejs\", and \"deno\" are mutually exclusive, but at least one of them has to be set when compiling to wasm32-unknown-unknown");

#[cfg(not(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
)))]
mod native;
#[cfg(not(all(
    target_arch = "wasm32",
    not(any(target_os = "emscripten", target_os = "wasi"))
)))]
pub use native::*;
