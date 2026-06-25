# PostMessageW only from hook callbacks

Hook callbacks must only call `PostMessageW` to signal state changes. They never call `lock()`, `unlock()`, or any other application logic directly.

`SendMessageW` is synchronous — it waits for the target WndProc to process the message before returning. Since both the hook callback and the WndProc run on the same main thread (driven by the hook pump), calling `SendMessageW` from a callback would deadlock. `PostMessageW` is async — it enqueues the message and returns immediately, well within the hook timeout.
