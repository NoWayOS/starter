// SPDX-License-Identifier: MIT OR Apache-2.0
//! `starter-virtio-input`, in QEMU: NoWayOS's driver PD for QEMU's keyboard and pointer.
//! All of it is NoWayOS's library; this program only starts it.

#![no_std]
#![no_main]

use nowayos_virtio_input::VirtioInputPd;
use sel4_microkit::protection_domain;

#[protection_domain]
fn init() -> VirtioInputPd {
    nowayos_virtio_input::init()
}
