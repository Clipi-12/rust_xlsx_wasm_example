#[cfg(feature = "javascript")]
mod javascript;
#[cfg(feature = "javascript")]
pub use javascript::*;
#[cfg(feature = "javascript")]
#[cfg(any(
    all(feature = "browser", feature = "node"),
    all(feature = "browser", feature = "deno"),
    all(feature = "nodejs", feature = "deno"),
    not(any(feature = "browser", feature = "nodejs", feature = "deno")),
))]
compile_error!("Feature \"browser\", \"nodejs\", and \"deno\" are mutually exclusive, but at least one of them has to be set when compiling to wasm32-unknown-unknown");

#[cfg(not(feature = "javascript"))]
mod native;
#[cfg(not(feature = "javascript"))]
pub use native::*;
