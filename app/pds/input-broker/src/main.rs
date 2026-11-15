// SPDX-License-Identifier: MIT OR Apache-2.0
//! `starter-input-broker`: NoWayOS's input broker PD: every input source's events checked and merged into one stream for the UI.
//! All of it is NoWayOS's library; this program only starts it.

#![no_std]
#![no_main]

use nowayos_input_broker::BrokerPd;
use sel4_microkit::protection_domain;

#[protection_domain]
fn init() -> BrokerPd {
    nowayos_input_broker::init()
}
