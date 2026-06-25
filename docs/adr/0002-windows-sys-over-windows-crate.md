# windows-sys over the windows crate

We use `windows-sys = "0.59"` (Microsoft's low-level Win32 bindings) rather than the higher-level `windows` crate. The `windows` crate's proc-macros and COM bindings have known compatibility issues with the `x86_64-pc-windows-gnu` (MinGW) target we cross-compile from WSL. `windows-sys` is thin, stable, and builds cleanly with GNU. We don't need COM or high-level abstractions — just function signatures and constants.
