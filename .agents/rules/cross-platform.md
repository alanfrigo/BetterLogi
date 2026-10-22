---
paths:
  - "crates/betterlogi-hook/**"
  - "crates/betterlogi-inject/**"
  - "crates/betterlogi-hid/**"
  - "crates/betterlogi-agent/src/autostart.rs"
  - "crates/betterlogi-agent/src/autostart/**"
  - "crates/betterlogi-agent/src/resume_windows.rs"
  - "crates/betterlogi-camera/**"
  - "crates/betterlogi-permissions/**"
---

# Platform / cfg-gated code — macOS-green is a trap

macOS-green proves **nothing** about `#[cfg(target_os = "linux")]` /
`windows` code. Recent agent failures that only showed up on CI Linux:

- Shadowing a crate-level constant with a local `const` of a different type
  (e.g. `LOGITECH_VENDOR_ID: u16` next to `use crate::LOGITECH_VENDOR_ID`
  which is `u32`) — E0255 / E0308, **only compiles on Linux**.
- Importing a name that only exists on another OS, or redefining one that
  master already exports from `lib.rs`.

When the diff touches any of:

- `crates/betterlogi-hook/src/linux.rs` / `windows/**`
- `crates/betterlogi-inject/src/inject/linux.rs` / `windows.rs`
- `crates/betterlogi-agent/src/autostart/linux.rs` / `windows.rs`, or
  `crates/betterlogi-agent/src/resume_windows.rs`
- `crates/betterlogi-camera/src/capture_linux.rs`, `capture_windows.rs`,
  `com_windows.rs`, `uvc_windows.rs`, `uvc_linux.rs`, `linux.rs`
- `crates/betterlogi-hid/src/transport.rs` or `transport/windows.rs`
- any `#[cfg(target_os = …)]` block, in any crate

you MUST either:

1. Cross-check with devenv when available:
   `devenv tasks run betterlogi:check-windows` (also
   `cargo xtask ci clippy-windows`), or
2. Manually re-read every changed cfg-gated file against **current master** for:
   - name collisions with existing `pub use` / `pub const` items
   - type mismatches (`u16` vs `u32`, `Option` arity, new enum fields)
   - call sites that gained args on master (e.g. `AppState::new`, `build_device_list`,
     `dispatch_action`) but the PR still uses the old signature

Do not claim "cross-platform green" without CI (or a local cross-lint) having
actually run those targets. `RUSTFLAGS=-D warnings` is global in CI — plain
warnings fail there too.

There is no Linux equivalent of the Windows task, and it cannot be complete if
there ever is: `betterlogi-camera`'s Linux backend needs kernel headers
(`v4l2-sys` wants `linux/videodev2.h`), so it does not cross-compile from macOS
at all. For a Linux-only change outside camera, rustup's
`aarch64-unknown-linux-musl` target covers the rest — but leave out `betterlogi`,
`betterlogi-cli` and `betterlogi-assets`, whose `ureq → ring` dependency needs a
cross C toolchain:

```sh
cargo clippy --target aarch64-unknown-linux-musl \
  -p betterlogi-hook -p betterlogi-inject -p betterlogi-hid -p betterlogi-hidpp \
  -p betterlogi-core -p betterlogi-agent -p betterlogi-agent-core -p betterlogi-ipc \
  -p betterlogi-permissions --all-targets -- -D warnings
```

Everything that recipe skips — camera on Linux above all — is CI's alone to
catch.
