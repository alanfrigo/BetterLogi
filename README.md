<h1 align="center">BetterLogi</h1>
<p align="center"><strong>A native, local-first alternative to Logitech Options+, written in Rust<br/>Logitech mice, keyboards, lights, and webcams over HID++ and UVC — no account, no telemetry</strong></p>

<div align="center">
    <a href="https://github.com/alanfrigo/BetterLogi/releases/latest">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/alanfrigo/BetterLogi?style=flat"></a>
    <a href="https://github.com/alanfrigo/BetterLogi/releases">
    <img alt="GitHub downloads" src="https://img.shields.io/github/downloads/alanfrigo/BetterLogi/total.svg?style=flat"></a>
    <a href="https://github.com/alanfrigo/BetterLogi/commits">
    <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/m/alanfrigo/BetterLogi?style=flat"></a>
    <a href="#license">
    <img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat"></a>
</div>

BetterLogi is an independent app built on [OpenLogi](https://github.com/AprilNEA/OpenLogi).
It keeps OpenLogi's device support and adds what the upstream project does not
ship yet:

- **Easy-Switch follow.** Link your mice to a keyboard: when you press an
  Easy-Switch key on the keyboard, the linked mice switch to the same computer.
  A mouse that is asleep or slow to answer is retried, so it does not stay
  behind.
- **Wayland-native overlays.** The Actions Ring and the Caps Lock, Num Lock,
  and backlight indicators open as overlay layer surfaces. On Hyprland and
  Omarchy they no longer open, tile, or take focus like an app window.
- **Optional lock-key indicator.** Turn the Caps Lock indicator off in
  **Settings → General → Lock key notifications**.
- **Releases from this repository.** Installers and in-app updates come from
  the [BetterLogi releases](https://github.com/alanfrigo/BetterLogi/releases).

Runs on Linux, macOS, and Windows. Linux, including Hyprland and Omarchy, is
the platform BetterLogi is developed on.

> [!WARNING]
> BetterLogi is under active development. Features and config may still change.

## Features

### Devices

- Logi Bolt and Unifying receivers, Bluetooth, and wired connections, with battery level and charge state
- Low-battery notices at the bottom of the screen, once per discharge
- An on-screen indicator when a keyboard turns Caps Lock on or off, plus Num Lock and Scroll Lock on Windows and Linux (optional)

### Easy-Switch

- See which computer is paired on each channel, with its OS and connection, and rename channels on devices that allow it
- **Mouse follow**: on the keyboard's **Device** tab, under **Easy-Switch**, turn on each mouse that should follow the keyboard. Pair channel *N* of the keyboard and of the mouse to the same computer; when the keyboard moves to channel 3, the mouse goes to channel 3 too
- Keyboards that announce a host change (MX Keys Mini, MX Keys S, and others with `ChangeHost` v2) are followed without touching their keys; older keyboards have their Easy-Switch keys diverted while the link is active

Run BetterLogi on every computer you switch between: the computer the keyboard
is leaving is the one that moves the mouse. See
[CONFIGURATION.md](docs/CONFIGURATION.md#linked-easy-switch-hosts).

### Buttons and actions

- Remap any button: a built-in action catalog, custom shortcuts, typed text, or multi-step workflows
- Separate tap and hold actions, and gestures on Middle Click, Back/Forward, DPI/ModeShift, the gesture button, and the haptic panel
- **Actions Ring**: eight actions in a ring around the cursor, with its own layout per application
- Bind media, emoji, screenshot, and other dedicated keyboard keys
- Per-application profiles that switch with the focused app

### Pointer and scrolling

- DPI presets, pointer speed, report rate, and Precision Mode
- SmartShift wheel mode, sensitivity, and scrolling force
- Scroll inversion, wheel resolution, smooth scrolling, and sideways scrolling with Back/Forward held

### Keyboard, lighting, and webcams

- Fn Lock, host OS layouts, backlight level, automatic brightness, and battery saving
- Static RGB on supported keyboards; power, brightness, and color temperature on Litra lights
- Any Logitech UVC webcam: live preview, every UVC control, and per-camera profiles

Feature coverage depends on what each device reports; the app shows only the
controls a device supports.

### Platform support

| Feature | Linux | macOS | Windows |
| --- | --- | --- | --- |
| Mouse button remapping and gestures | ✓ | ✓ | ✓ |
| Easy-Switch mouse follow | ✓ | ✓ | ✓ |
| Keyboard F-key remapping | — | ✓ | ✓ |
| Per-application profiles | X11, wlroots (sway, Hyprland), GNOME with the [companion extension](crates/betterlogi-hook/gnome-shell-extension/README.md) | ✓ | ✓ |
| Actions Ring and indicators as overlays | Wayland layer shell (Hyprland, sway, KDE), window elsewhere | ✓ | ✓ |
| Media keys | D-Bus MPRIS | Native | Native |
| In-app updates | — (re-run the installer) | Sparkle | Built-in |

## Install

> [!IMPORTANT]
> Quit **Logi Options+** (and Solaar on Linux) first. Only one application can own a receiver at a time.

Download the package for your system from the
[latest release](https://github.com/alanfrigo/BetterLogi/releases/latest).
Every download has a minisign signature and an entry in `SHA256SUMS`.

| Platform | System | Architecture | Package |
| --- | --- | --- | --- |
| Linux | GLIBC 2.35 or newer | x86_64, arm64 | `.deb`, `.rpm`, `.pkg.tar.zst`, Flatpak |
| macOS | macOS 13 or later | Apple silicon, Intel | `.dmg` |
| Windows | Windows 11 | x86_64, arm64 | `.msi`, portable `.zip` |

On Linux, the installer picks the right package for your distribution, checks
its signature, and starts the agent (install `minisign` first):

```sh
curl -fsSL https://github.com/alanfrigo/BetterLogi/releases/latest/download/install.sh | sh
```

See [INSTALL-linux.md](docs/INSTALL-linux.md) to read the script first, and for
Flatpak, NixOS, and distributions without systemd. On macOS, open BetterLogi and
grant **Accessibility** and **Input Monitoring** when macOS asks.

To build from source, see [DEVELOPMENT.md](docs/DEVELOPMENT.md).

### Updates

On macOS and Windows, with **Settings → Updates → Check for updates** on,
BetterLogi checks [this repository's releases](https://github.com/alanfrigo/BetterLogi/releases)
once per launch and installs only an update whose signature matches the key
built into the app. On Linux the packages are not updated in place: run the
install command above again to move to the newest release.

### Omarchy

BetterLogi draws the Actions Ring and its indicators itself as Wayland overlay
surfaces, so the [omarchy-openlogi](https://github.com/alanfrigo/omarchy-openlogi)
plugin is no longer needed for them. Uninstall the plugin
(`bash scripts/uninstall.sh` in its checkout) before installing BetterLogi.

## Usage

- CLI: [USAGE.md](docs/USAGE.md)
- Configuration (plain TOML): [CONFIGURATION.md](docs/CONFIGURATION.md)
- Development: [DEVELOPMENT.md](docs/DEVELOPMENT.md)

## Acknowledgments

BetterLogi is a fork of [OpenLogi](https://github.com/AprilNEA/OpenLogi) by
[@AprilNEA](https://github.com/AprilNEA) and its contributors, including
[@davidbudnick](https://github.com/davidbudnick) (Windows, cameras, i18n) and
[@cserby](https://github.com/cserby) (Linux). It also builds on
[Solaar](https://github.com/pwr-Solaar/Solaar) by [@pwr](https://github.com/pwr)
and [`hidpp`](https://crates.io/crates/hidpp) by [@lus](https://github.com/lus).

## License

The code is dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. `crates/betterlogi-hidpp` is a vendored fork of
[`hidpp`](https://crates.io/crates/hidpp), licensed 0BSD.

The files under [`design/`](design/) are OpenLogi's brand assets, © 2026
AprilNEA, all rights reserved, and are not covered by these licenses; see
[`design/LICENSE`](design/LICENSE).

---

**Not affiliated with Logitech or with the OpenLogi project.** "Logitech", "MX
Master", and "Options+" are trademarks of Logitech International S.A.
