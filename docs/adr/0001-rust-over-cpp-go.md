# Rust over C++ and Go

We chose Rust for the implementation. Go's GC pauses can exceed the ~200ms hook callback timeout, causing Windows to silently remove the hook — blocking silently stops working with no error. Tested and confirmed broken in practice. The C++ prototype works, but Rust gives memory safety at the hook callback level; a crash or memory corruption there can freeze the entire OS input pipeline, which is a worse failure mode than any other kind of crash. Rust has no GC, no runtime, compiles to bare `extern "system" fn` pointers, and cross-compiles cleanly from WSL via `x86_64-pc-windows-gnu`.

## Considered Options

- **Go** — rejected: GC pauses exceed hook timeout
- **C++** — acceptable but rejected: prototype exists; Rust gives memory safety at the hook level for no runtime cost
- **Rust** — chosen
