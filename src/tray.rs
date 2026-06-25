// Wraith — system tray icon, context menu, balloon notifications
// Step 3: TrayIcon lifecycle via Shell_NotifyIconW

pub struct TrayIcon {
    // TODO: hwnd, uid, current lock state
}

impl TrayIcon {
    pub fn new(_hwnd: usize) -> Self {
        // TODO: Shell_NotifyIconW(NIM_ADD, ...)
        TrayIcon {}
    }

    pub fn set_locked(&mut self, _locked: bool) {
        // TODO: Shell_NotifyIconW(NIM_MODIFY, ...) — swap icon, update tooltip
    }

    pub fn show_balloon(&self, _title: &str, _msg: &str) {
        // TODO: Shell_NotifyIconW(NIM_MODIFY, ...) with NIF_INFO
    }

    pub fn show_menu(&self, _hwnd: usize) {
        // TODO: CreatePopupMenu, AppendMenuW, TrackPopupMenu
        // Items: Lock/Unlock, Auto-start (checked), separator, Exit
    }

    pub fn destroy(&mut self) {
        // TODO: Shell_NotifyIconW(NIM_DELETE, ...)
    }
}
