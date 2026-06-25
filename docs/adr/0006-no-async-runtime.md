# No async runtime

We use `std::thread::spawn` for the update checker, not Tokio or async-std. The update checker is a single one-shot HTTP request on a background thread. A plain OS thread is sufficient; `PostMessageW` bridges the result back to the main thread cleanly. An async runtime would add significant binary size and compile complexity for zero practical benefit. Target binary size is ~50–100 KB.
