# betterlogi-hid — the host-side HID backend

The `HidBackend` implementation for this machine: `async-hid` transport, the
Windows composite channel, macOS Input Monitoring, the on-disk probe cache,
plus `host`, which supplies it to `betterlogi-device`'s entry points.

The layer's contract — the `HidBackend` seam, the device-kind vocabularies,
enumeration grace — is [`crates/betterlogi-device/AGENTS.md`](../betterlogi-device/AGENTS.md);
read it before changing this crate. Serde types here ride the IPC wire
([`crates/betterlogi-ipc/AGENTS.md`](../betterlogi-ipc/AGENTS.md)), and the
cfg-gated platform code falls under
[`.agents/rules/cross-platform.md`](../../.agents/rules/cross-platform.md).
