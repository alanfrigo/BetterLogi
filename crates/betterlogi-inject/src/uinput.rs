//! Creates the uinput devices BetterLogi uses on Linux.
//!
//! Their names, and the check that recognizes them, live in
//! [`betterlogi_core::uinput`] so the permissions probe can use them without
//! linking evdev; they are re-exported here for the hook and the injector.

use std::io;

pub use betterlogi_core::uinput::{Device, is_own_device};
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};

/// Start a uinput builder for `device` that already carries its name.
///
/// # Errors
///
/// Returns the error from opening `/dev/uinput`.
pub fn builder(device: Device) -> io::Result<VirtualDeviceBuilder<'static>> {
    Ok(VirtualDevice::builder()?.name(device.name()))
}
