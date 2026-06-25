# Global atomics over Mutex for hook-shared state

Hook callbacks are bare `extern "system" fn` pointers — they cannot capture environment. All state shared with callbacks must be global. We use `AtomicBool`, `AtomicUsize`, and `AtomicU32` rather than `Mutex<T>`.

The hook has a hard ~200ms return deadline. Any blocking call in the callback — including a contended `Mutex::lock()` — can cause Windows to silently remove the hook, stopping input blocking with no error or notification. Atomics are lock-free and cannot block. The values being shared (a bool, two pointer-sized handles, a tick count) map perfectly to atomic types.
