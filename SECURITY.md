# Security Policy

## Scope

Wraith installs `WH_KEYBOARD_LL` and `WH_MOUSE_LL` global hooks that intercept all system input. Security issues in this surface area are taken seriously.

**In scope:**
- Wraith blocking synthetic (injected) input it should pass through
- Wraith failing to block physical input it should block
- The update checker sending or receiving unexpected data
- Registry keys written by Wraith persisting after uninstall
- Privilege escalation through Wraith's hook callbacks

**Out of scope:**
- `Ctrl+Alt+Del` not being blockable — this is a Windows kernel constraint (Secure Attention Sequence), not a Wraith bug
- Other processes with Administrator privileges being able to terminate Wraith — this is the Windows security model and an intentional escape hatch
- SmartScreen warnings on the unsigned binary

## Reporting a Vulnerability

Open a [GitHub Issue](https://github.com/shadow-dragon-2002/Wraith/issues) marked with the `security` label, or email the maintainer directly if the issue is sensitive enough that public disclosure would be harmful before a fix is available.

Include:
- A description of the vulnerability
- Steps to reproduce
- The Windows version and Wraith version affected

## Known Security Behaviour

The following are intentional design decisions, not vulnerabilities:

| Behaviour | Reason |
|-----------|--------|
| Synthetic input (`SendInput`, RDP, Parsec, AHK) passes through while locked | This is the core feature — blocking only physical hardware while letting AI/automation work |
| `DisableTaskMgr` is set in HKCU on lock | Prevents Ctrl+Alt+Del → Task Manager as an escape path; cleared on unlock and on next Wraith startup after a crash |
| Wraith runs `asInvoker` (no UAC prompt) | `WH_KEYBOARD_LL` does not require elevation; change to `requireAdministrator` in `wraith.manifest` if you want to prevent standard users from terminating the process |
