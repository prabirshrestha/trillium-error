# trillium-error

This crate adds support for error handling in [trillium](https://trillium.rs) web framework.

Due to limitations in Rust, error handling is currently not supported in trillium. When the language
adds capability to express bounds for `for<'a> Fn(&'a Conn) -> impl Future<Output=…> + 'a`, trillium
will add first class support for error handling. For more details please refer to the discussion
[here](https://github.com/trillium-rs/trillium/discussions/31). Until then `trillium-error` provides
a proc macro to help write handlers with error.

# Usage

Refer to the [docs](https://docs.rs/trillium-error/latest/trillium_error/).

# LICENSE
License under Apache 2.0 or MIT
