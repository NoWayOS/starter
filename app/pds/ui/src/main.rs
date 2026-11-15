// SPDX-License-Identifier: MIT OR Apache-2.0
//! `starter-ui`: the UI protection domain - the application on the device.
//!
//! It runs `starter_app::App` through NoWayOS's UI loop (`nowayos-ui-loop`),
//! which does everything else: the display contract, the framebuffer it
//! shares with the display PD, the renderer and the rings. This file is all
//! a product's UI PD adds, and you will rarely change it: your work is in
//! `starter-app`.
//!
//! No `unsafe` code, no device, no interrupt: its role in the grants file
//! (`systems/starter.grants`) is `ui`, and the system gives it nothing but
//! its rings and the framebuffer.
//!
//! Input arrives from the input broker as canonical `InputEvent` records.
//! After each batch it draws only what changed and presents those
//! rectangles; once the display has shown them, it tells the broker it is
//! ready for more. It prints which screen is showing and which button was
//! pressed, and never anything typed.

#![no_std]
#![no_main]

use nowayos_display::{DeviceScale, UiEvent};
use nowayos_render_eg::{DEFAULT_BYTES, DEFAULT_SLOTS, Glyphs, Slot};
use nowayos_ui_loop::{LoopError, UiChannels, UiLoop};
use sel4_microkit::{
    Channel, ChannelSet, Handler, Infallible, NullHandler, debug_println, protection_domain,
};
use starter_app::App;

/// The channel to the display PD.
const DISPLAY: Channel = Channel::new(0);

/// The channel to the input broker.
const INPUT: Channel = Channel::new(1);

#[protection_domain]
fn init() -> NullHandler {
    debug_println!("STARTER: ui: starting");
    // The glyph store's storage lives in this frame, which runs the event
    // loop and never returns: a PD has no heap. The system gives the PD a
    // stack to hold it.
    let mut pixels = [0_u8; DEFAULT_BYTES];
    let mut slots = [Slot::EMPTY; DEFAULT_SLOTS];
    let glyphs = Glyphs::new(&mut pixels, &mut slots);
    let state = match start(glyphs) {
        Ok(state) => Some(state),
        Err(error) => {
            debug_println!("STARTER: ui: gave up: {error}");
            None
        }
    };
    run_forever(&mut UiPd { state })
}

/// rust-sel4's event loop for `pd`, which never returns.
#[allow(
    unreachable_code,
    reason = "Handler::run never returns: its type, Result<Never, Infallible>, has no value"
)]
fn run_forever(pd: &mut UiPd<'_>) -> ! {
    match pd.run() {
        Ok(never) => match never {},
        Err(infallible) => match infallible {},
    }
}

fn start(glyphs: Glyphs<'_>) -> Result<State<'_>, LoopError> {
    let channels = UiChannels::bind()?;
    let ui = UiLoop::new(channels.frame_len());
    Ok(State {
        channels,
        ui,
        glyphs,
        app: None,
        said: "",
    })
}

/// Everything the UI PD holds once started.
struct State<'g> {
    channels: UiChannels,
    ui: UiLoop,
    /// Where its text's glyphs come from: drawn from the outlines, kept.
    glyphs: Glyphs<'g>,
    /// The application, once the display has announced its layout.
    app: Option<App>,
    /// The screen last named in the log.
    said: &'static str,
}

impl State<'_> {
    /// Takes every record from the display and acts on it.
    fn serve(&mut self) -> Result<(), LoopError> {
        while let Some(record) = self.channels.next_from_display()? {
            match self.ui.receive(&record)? {
                UiEvent::Configured(layout) => {
                    let scale = self.ui.scale().unwrap_or(DeviceScale::ONE);
                    debug_println!(
                        "STARTER: ui: the display is {}x{} at scale {scale}",
                        layout.width(),
                        layout.height()
                    );
                    let viewport = self.ui.viewport().ok_or(LoopError::Ring)?;
                    self.app =
                        Some(App::new(self.ui.metrics(), viewport).map_err(|_| LoopError::Ring)?);
                    self.present()?;
                }
                UiEvent::PresentDone(_) => INPUT.notify(),
            }
        }
        Ok(())
    }

    /// Takes every input record waiting, then draws and presents the
    /// result.
    fn take_input(&mut self) -> Result<(), LoopError> {
        let mut any = false;
        while let Some(record) = self.channels.next_input()? {
            any = true;
            match record {
                Ok(event) => {
                    let Some(app) = self.app.as_mut() else {
                        continue;
                    };
                    if let Some(button) = app.handle(event).map_err(|_| LoopError::Ring)? {
                        debug_println!("STARTER: ui: pressed {button}");
                    }
                }
                Err(error) => debug_println!("STARTER: ui: input record rejected: {error}"),
            }
        }
        if any {
            self.present()?;
        }
        Ok(())
    }

    /// Draws what changed - the whole screen, the first time - and
    /// presents it, naming the screen when it is a new one.
    fn present(&mut self) -> Result<(), LoopError> {
        let Some(app) = self.app.as_mut() else {
            return Ok(());
        };
        let present =
            self.ui
                .draw_dirty(app.tree_mut(), self.channels.frame(), &mut self.glyphs)?;
        self.channels.send(&present)?;
        DISPLAY.notify();
        if app.screen() != self.said {
            self.said = app.screen();
            debug_println!("STARTER: ui: showing {}", self.said);
        }
        Ok(())
    }
}

/// The UI PD: `None` once it has given up.
struct UiPd<'g> {
    state: Option<State<'g>>,
}

impl Handler for UiPd<'_> {
    type Error = Infallible;

    fn notified(&mut self, channels: ChannelSet) -> Result<(), Self::Error> {
        let Some(state) = self.state.as_mut() else {
            return Ok(());
        };
        let mut result = Ok(());
        if channels.contains(DISPLAY) {
            result = state.serve();
        }
        if result.is_ok() && channels.contains(INPUT) {
            result = state.take_input();
        }
        if let Err(error) = result {
            debug_println!("STARTER: ui: gave up: {error}");
            self.state = None;
        }
        Ok(())
    }
}
