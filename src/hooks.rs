// Wraith — WH_KEYBOARD_LL / WH_MOUSE_LL hooks + global atomics
// Step 4: install/uninstall, keyboard_proc, mouse_proc

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize};

pub static LOCKED:      AtomicBool  = AtomicBool::new(false);
pub static KB_HOOK:     AtomicUsize = AtomicUsize::new(0); // HHOOK as usize
pub static MOUSE_HOOK:  AtomicUsize = AtomicUsize::new(0); // HHOOK as usize
pub static APP_HWND:    AtomicUsize = AtomicUsize::new(0); // HWND as usize
pub static PANIC_START: AtomicU32   = AtomicU32::new(0);   // GetTickCount() snapshot

pub fn install(_hwnd: usize) -> Result<(), &'static str> {
    // TODO: SetWindowsHookExW(WH_KEYBOARD_LL, keyboard_proc, NULL, 0)
    // TODO: SetWindowsHookExW(WH_MOUSE_LL, mouse_proc, NULL, 0)
    Ok(())
}

pub fn uninstall() {
    // TODO: UnhookWindowsHookEx(KB_HOOK), UnhookWindowsHookEx(MOUSE_HOOK)
}

// unsafe extern "system" fn keyboard_proc(code: i32, wparam: usize, lparam: isize) -> isize {
//     TODO:
//     1. if LLKHF_INJECTED set → CallNextHookEx (pass through)
//     2. if lock combo → PostMessageW(ID_LOCK) + consume
//     3. if unlock combo → PostMessageW(ID_UNLOCK) + consume
//     4. if LOCKED → return 1 (block)
//     5. else → CallNextHookEx
// }

// unsafe extern "system" fn mouse_proc(code: i32, wparam: usize, lparam: isize) -> isize {
//     TODO:
//     1. if LLMHF_INJECTED set → CallNextHookEx
//     2. if LOCKED → return 1 (block)
//     3. else → CallNextHookEx
// }
