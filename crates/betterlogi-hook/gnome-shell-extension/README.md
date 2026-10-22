# BetterLogi Frontmost Window — GNOME Shell extension

GNOME (Mutter) does not let ordinary clients see which window is focused on
Wayland, and it implements neither `wlr-foreign-toplevel` nor a focused-window
portal. This minimal extension bridges that gap: it exports the WM_CLASS of the
focused window over D-Bus and emits every native focus change so BetterLogi's
`gnome-shell` backend can drive per-app mouse-profile switching without
polling.

It reads only `global.display.focus_window.get_wm_class()`. No titles, no window
contents, no input, no UI.

## D-Bus surface

- name: `org.betterlogi.Frontmost`
- path: `/org/betterlogi/Frontmost`
- method: `GetFocusedWmClass() -> s` (empty string when nothing is focused)
- signal: `FocusedWmClassChanged(s)` (emitted on each GNOME focus change)

## Install

```sh
UUID=betterlogi-frontmost@betterlogi.dev
DEST="$HOME/.local/share/gnome-shell/extensions/$UUID"
mkdir -p "$DEST"
cp metadata.json extension.js "$DEST"/
```

On Wayland the shell cannot be reloaded in place, so **log out and back in** to
let GNOME pick up the newly added extension, then enable it:

```sh
gnome-extensions enable "$UUID"
gnome-extensions info "$UUID"   # State should be ACTIVE
```

## Verify

```sh
# Introspect the service:
busctl --user introspect org.betterlogi.Frontmost /org/betterlogi/Frontmost

# Focus a window, then query it:
gdbus call --session \
  -d org.betterlogi.Frontmost \
  -o /org/betterlogi/Frontmost \
  -m org.betterlogi.Frontmost.GetFocusedWmClass

# Watch focus changes:
gdbus monitor --session \
  -d org.betterlogi.Frontmost \
  -o /org/betterlogi/Frontmost
```

If the call prints the focused window's WM_CLASS and the monitor reports
`FocusedWmClassChanged` while switching windows, BetterLogi's GNOME backend will
pick it up automatically the next time the hook starts.

## Notes

- The `shell-version` list in `metadata.json` covers GNOME 45–50. Newer GNOME
  releases may need an added entry; the API used here (`Gio.DBusExportedObject`,
  `global.display.focus_window`, `Meta.Window.get_wm_class`) has been stable
  across these versions.
- The extension name/UUID and the D-Bus name (`org.betterlogi.*`) are placeholders
  that should track the project's namespace; if they change, update the matching
  constants in `crates/betterlogi-hook/src/linux/gnome_shell.rs`.
