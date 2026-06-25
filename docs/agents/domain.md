# Domain Docs

This is a **single-context** repository.

## Domain glossary

`CONTEXT.md` at the repo root. This is the canonical source of domain vocabulary. Skills must use the terms defined there. `/grill-with-docs` and `/improve-codebase-architecture` update it inline as decisions crystallise.

Rules for reading:
- Use the exact terms from the **Language** section when naming modules, functions, tests, and issues
- Do not add implementation details to `CONTEXT.md` — it is a glossary only
- When a term is fuzzy or missing, surface it during a grill session before naming things

## Architecture decisions

`docs/adr/` at the repo root. Individual files named `NNNN-slug.md`.

Current ADRs:
- `0001-rust-over-cpp-go.md` — why Rust (not Go or C++)
- `0002-windows-sys-over-windows-crate.md` — why windows-sys (not windows crate)
- `0003-global-atomics-over-mutex.md` — why atomics (not Mutex) for hook state
- `0004-postmessagew-from-hooks.md` — why PostMessageW (not SendMessageW or direct calls)
- `0005-panic-abort-in-release.md` — why panic=abort in release profile
- `0006-no-async-runtime.md` — why std::thread (not Tokio)

Rules for reading:
- Do not re-litigate decisions already recorded in an ADR unless the friction is significant enough to warrant reopening it
- When proposing a change that contradicts an ADR, flag it explicitly ("contradicts ADR-0003 — worth reopening because…")
- New ADRs: only when the decision is hard to reverse, surprising without context, and the result of a real trade-off
