//! Presentation code shared by BetterLogi's two GPUI processes: the settings app
//! (`betterlogi-desktop`) and the Actions Ring overlay helper (`betterlogi-overlay`).
//!
//! Only what both genuinely link belongs here. The overlay is a pure IPC client
//! with no settings UI, and this crate is what keeps that true: it depends on
//! `gpui` but not on `gpui-component`, so nothing the settings app pulls in can
//! reach the overlay through the back door.

pub mod action_icons;
pub mod color;
