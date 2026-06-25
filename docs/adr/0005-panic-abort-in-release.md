# panic = "abort" in release profile

The release profile sets `panic = "abort"`. Rust's default unwind behaviour in `extern "system"` functions is undefined behaviour — unwinding across an FFI boundary is unsound. With `abort`, a panic terminates the process immediately rather than corrupting state or causing UB in the hook. This also reduces binary size by eliminating unwind tables.
