use std::sync::OnceLock;
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::System::WindowsProgramming::GetPrivateProfileIntW;

static CONFIG: OnceLock<Config> = OnceLock::new();

const DEFAULT_LOCK_MODS: i32 = 7;   // MOD_ALT|MOD_CONTROL|MOD_SHIFT = 1|2|4
const DEFAULT_LOCK_VK: i32 = 76;    // 'L'
const DEFAULT_UNLOCK_MODS: i32 = 7;
const DEFAULT_UNLOCK_VK: i32 = 85;  // 'U'
const DEFAULT_PANIC_VK: i32 = 27;   // VK_ESCAPE
const DEFAULT_LOCK_ON_START: i32 = 0;

pub struct Config {
    pub lock_mods: u32,
    pub lock_vk: u32,
    pub unlock_mods: u32,
    pub unlock_vk: u32,
    pub panic_vk: u32,
    pub lock_on_start: bool,
}

impl Config {
    pub fn load() -> Self {
        let ini = exe_relative("wraith.ini");
        let sec = crate::to_wide("Wraith");

        macro_rules! get_int {
            ($key:expr, $default:expr) => {{
                let k = crate::to_wide($key);
                unsafe {
                    GetPrivateProfileIntW(sec.as_ptr(), k.as_ptr(), $default, ini.as_ptr()) as u32
                }
            }};
        }

        Config {
            lock_mods:     get_int!("LockModifiers",  DEFAULT_LOCK_MODS),
            lock_vk:       get_int!("LockKey",         DEFAULT_LOCK_VK),
            unlock_mods:   get_int!("UnlockModifiers", DEFAULT_UNLOCK_MODS),
            unlock_vk:     get_int!("UnlockKey",       DEFAULT_UNLOCK_VK),
            panic_vk:      get_int!("PanicKey",        DEFAULT_PANIC_VK),
            lock_on_start: get_int!("LockOnStart",     DEFAULT_LOCK_ON_START) != 0,
        }
    }

    pub fn get() -> &'static Self {
        CONFIG.get_or_init(Self::load)
    }
}

fn exe_relative(filename: &str) -> Vec<u16> {
    let mut buf = [0u16; 520];
    let len = unsafe { GetModuleFileNameW(std::ptr::null_mut(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    let dir_end = buf[..len]
        .iter()
        .rposition(|&c| c == b'\\' as u16 || c == b'/' as u16)
        .map(|i| i + 1)
        .unwrap_or(0);
    let mut path = buf[..dir_end].to_vec();
    path.extend(crate::to_wide(filename));
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_ini_docs() {
        assert_eq!(DEFAULT_LOCK_MODS, 7);
        assert_eq!(DEFAULT_LOCK_VK, 76);
        assert_eq!(DEFAULT_UNLOCK_MODS, 7);
        assert_eq!(DEFAULT_UNLOCK_VK, 85);
        assert_eq!(DEFAULT_PANIC_VK, 27);
        assert_eq!(DEFAULT_LOCK_ON_START, 0);
    }
}
