# Wraith — Product Requirements

> Feed this to `/grill-with-docs` then `/to-prd`.

---

## Problem Statement

Modern AI workflows (Claude in Chrome, computer-use agents, AutoHotkey, Parsec/RDP sessions) require a PC to stay active — screen on, session alive — while the user steps away. Windows' built-in solutions all fail this:

- **Lock screen** (`Win+L`): kills the active session, disconnects AI tools
- **Screensaver / display sleep**: interrupts visual AI workflows
- **Walk away unprotected**: anyone at the desk can interact with the running session

There is no native Windows mechanism to selectively block physical hardware input while preserving software-originated input. `BlockInput()` blocks everything including AI tool output.

**Solution:** Windows tags every input event with an injected flag (`LLKHF_INJECTED` / `LLMHF_INJECTED`) when it originates from software rather than hardware. Wraith intercepts at the hook level, passes injected events through, and blocks physical events while locked.

---

## User Persona

**Primary — "The AI Power User"**
- Runs long-duration AI agent tasks on Windows laptop/desktop
- Works in semi-public environments (office, café, co-working space)
- Cannot lock machine while AI works, cannot risk others touching keyboard/mouse
- Values: reliability above all — silent failure is worse than no tool at all

**Secondary — "The Automation Developer"**
- Builds/runs automated Windows testing or RPA workflows
- Needs input isolation without killing the automation session
- Cares about configurability (custom hotkeys, auto-start, INI config)

---

## User Stories

### US-01 — Physical Input Blocking
As a user, I want physical keyboard and mouse input suppressed when locked, so nobody at my machine can interfere with my running AI session.
- Given locked → When physical key pressed → no character appears in focused window
- Given locked → When physical mouse clicked → click produces no effect
- Given locked → When `SendInput` called programmatically → input reaches target normally
- Given locked → When Chrome extension injects keypress → keypress reaches browser

### US-02 — Hotkey Lock/Unlock
As a user, I want to lock and unlock with a keyboard shortcut.
- Given unlocked → When Ctrl+Shift+Alt+L pressed → locked within 100ms
- Given locked → When Ctrl+Shift+Alt+U pressed → unlocked within 100ms
- Given locked → When unlock combo pressed → combo consumed, not forwarded to apps
- Given any state → When combo triggered via `SendInput` → combo still processed

### US-03 — Panic Unlock
As a user, I want a physical fallback so I'm never locked out.
- Given locked → When Esc held 3 continuous seconds → unlocks
- Given locked → When Esc held <3s then released → stays locked, timer resets

### US-04 — AI / Synthetic Input Passthrough
As an AI tool, I need `SendInput`-based events to pass through regardless of lock state.
- Given locked → When `SendInput` sends keyboard events → LLKHF_INJECTED set → passes through
- Given locked → When `SendInput` sends mouse events → LLMHF_INJECTED set → passes through
- Given locked → When Claude in Chrome injects input via CDP → reaches the page
- Given locked → When Parsec/RDP sends input → passes through (injected flag set by RDP stack)

### US-05 — Sleep / Display Prevention
As a user, I want the screen to stay on and system awake while Wraith runs.
- Given running → When inactivity timeout triggers → system stays awake, display stays on
- Given closed → When timeout triggers → normal Windows sleep behavior resumes

### US-06 — System Tray
As a user, I want a tray icon showing lock state with a quick-access menu.
- Given running → tray icon visible in notification area
- Given lock/unlock → tray icon updates within 200ms
- Given right-click → menu shows Lock/Unlock, Auto-start toggle, Exit
- Given double-click → toggles lock state

### US-07 — Configurable Hotkeys
As a user, I want to customize hotkeys via config file.
- Given wraith.ini with custom values → custom hotkeys used on launch
- Given missing wraith.ini → defaults used, no crash
- Given invalid values → field-level defaults, no crash

### US-08 — Auto-start
As a user, I want optional auto-start with Windows.
- Given auto-start enabled → Windows boot → Wraith launches
- Given toggle off in tray → no longer starts on boot
- State stored in registry (not INI)

### US-09 — Update Notifications
As a user, I want to know when a newer version is available.
- Given newer version on GitHub → tray balloon on start
- Given latest version → no notification
- Given GitHub unreachable → silent fail, no crash

### US-10 — Single Instance
As the system, only one Wraith instance should run at a time.
- Given already running → second launch exits immediately, no error dialog

---

## Non-Functional Requirements

**Performance:**
- Hook callbacks return in <5ms (hard limit ~200ms before silent unhook)
- Lock/Unlock transition <100ms
- Binary target: <500 KB, no external runtime

**Reliability:**
- Hooks remain installed for process lifetime
- Clean exit always: hooks unregistered, sleep state restored, tray icon removed
- No crash on missing/invalid wraith.ini

**Compatibility:**
- Windows 10 (1903+) and Windows 11, x64 only
- Runs under UAC (manifest: `requireAdministrator`)
- DPI awareness: PerMonitorV2

**Security:**
- Named mutex prevents multiple instances
- No network calls except GitHub API for version check
- MIT licensed, fully open source, auditable

---

## Out of Scope (v1)

- Per-device filtering (requires Interception kernel driver)
- Blocking Ctrl+Alt+Del (impossible in user mode)
- macOS / Linux support (future — see docs/PLATFORM-ROADMAP.md)
- GUI settings panel (INI file sufficient for v1)
- Password-protected unlock
- Remote unlock
- Logging / audit trail
- Per-application rules
