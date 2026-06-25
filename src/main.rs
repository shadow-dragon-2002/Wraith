// Wraith — entry point
// Step 1: single-instance mutex + HWND_MESSAGE window + GetMessageW loop

mod app;
mod config;
mod hooks;
mod tray;
mod updater;

fn main() {
    // TODO (Step 1): CreateMutexW("Global\\WraithSingleInstance") — exit if ERROR_ALREADY_EXISTS
    // TODO (Step 2): Config::load() into OnceLock
    // TODO (Step 1): RegisterClassExW + CreateWindowExW(HWND_MESSAGE)
    // TODO (Step 1): hooks::APP_HWND.store(hwnd)
    // TODO (Step 3): TrayIcon::new(hwnd)
    // TODO (Step 4): hooks::install(hwnd)
    // TODO (Step 5): if Config::get().lock_on_start { app::lock() }
    // TODO (Step 8): updater::spawn(hwnd)
    // TODO (Step 1): GetMessageW loop
}
