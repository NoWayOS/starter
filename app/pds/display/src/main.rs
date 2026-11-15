// SPDX-License-Identifier: MIT OR Apache-2.0
//! `starter-display`, in QEMU: NoWayOS's display PD, showing each frame the UI presents on QEMU's screen.
//! All of it is NoWayOS's library; this program only starts it.

#![no_std]
#![no_main]

use nowayos_display_ramfb::DisplayPd;
use sel4_microkit::protection_domain;

#[protection_domain]
fn init() -> DisplayPd {
    nowayos_display_ramfb::init_quiet()
}
