// Wraith — lock/unlock logic, WndProc, auto-start
// Step 5: lock() / unlock() + SetThreadExecutionState
// Step 7: set_autostart() / is_autostart()

pub fn lock() {
    // TODO: LOCKED.store(true), SetThreadExecutionState, update tray
}

pub fn unlock() {
    // TODO: LOCKED.store(false), SetThreadExecutionState(ES_CONTINUOUS), update tray
}

pub fn toggle() {
    // TODO: lock() if unlocked, unlock() if locked
}

pub fn set_autostart(_enable: bool) {
    // TODO: write HKCU\...\Run registry key
}

pub fn is_autostart() -> bool {
    // TODO: read HKCU\...\Run registry key
    false
}

// pub unsafe extern "system" fn wnd_proc(...) -> LRESULT { ... }
// TODO (Step 1): handle WM_COMMAND, WM_TRAY_MSG, WM_TIMER, WM_UPDATE_RESULT, WM_DESTROY
