// SPDX-License-Identifier: MIT OR Apache-2.0
//! `make dev`'s program: the application running on this machine, shown in a
//! browser window (NoWayOS's dev window) and taking its clicks and keys.
//!
//! It is the same `App` the device runs, built for the host. Nothing here
//! goes on a device.
//!
//! ```text
//! starter-dev [--screen NAME] [--listen ADDRESS:PORT] [--session FILE]
//! ```
//!
//! `--screen NAME` opens the application on that screen (a TSX file's name):
//! for looking at a screen you have not wired in yet.

use std::env;
use std::process::ExitCode;

use nowayos_dev_window::ui::{Ui, UiApplication};
use nowayos_dev_window::{Options, Server};
use nowayos_display::{DeviceScale, Rect};
use nowayos_input::InputEvent;
use nowayos_render_eg::{Rgb888, Theme};
use nowayos_ui::Tree;
use starter_app::App;
use starter_app::generated::{CAPACITY, TEXT};

/// The screen: the Raspberry Pi 5's HDMI monitor, 1920 x 1080, drawn at
/// device scale 2 - what QEMU shows too.
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const SCALE: DeviceScale = DeviceScale::TWO;

/// The application, as the window drives it.
struct Shown(App);

impl UiApplication<CAPACITY, TEXT> for Shown {
    fn tree(&self) -> &Tree<CAPACITY, TEXT> {
        self.0.tree()
    }

    fn handle(&mut self, event: InputEvent) {
        // A screen that cannot be built leaves the last one showing.
        let _ = self.0.handle(event);
    }

    fn status(&self) -> String {
        self.0.screen().to_owned()
    }
}

fn main() -> ExitCode {
    match start(env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("starter-dev: {message}");
            ExitCode::FAILURE
        }
    }
}

fn start(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut screen = None;
    let mut rest = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--screen" {
            screen = Some(args.next().ok_or("--screen needs a screen's name")?);
        } else {
            rest.push(arg);
        }
    }
    let options = Options::from_args("Starter", rest.into_iter())?;
    let server =
        Server::bind(options.clone()).map_err(|err| format!("{}: {err}", options.listen))?;
    println!("open http://localhost:{}/", options.listen.port());
    server
        .run(|| {
            let metrics = Theme::<Rgb888>::light().metrics_at(SCALE);
            let mut app =
                App::new(metrics, Rect::new(0, 0, WIDTH, HEIGHT)).map_err(|err| err.to_string())?;
            if let Some(name) = &screen {
                app.show(name)
                    .map_err(|_| format!("no screen called {name:?} (a file in tsx/)"))?;
            }
            Ok(Ui::new(Shown(app), WIDTH, HEIGHT))
        })
        .map_err(|err| err.to_string())
}
