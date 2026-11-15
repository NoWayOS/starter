// SPDX-License-Identifier: MIT OR Apache-2.0
//! The application driven as a person drives it, on the host: keys in,
//! screens out. `make test` runs it; nothing is booted.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test code: a failed step fails the test"
)]

use nowayos_display::Rect;
use nowayos_input::{InputEvent, KeyCode};
use nowayos_render_eg::{Rgb888, Theme};
use starter_app::App;
use starter_app::generated::hello::hello_screen as hello;
use starter_app::generated::welcome::welcome_screen as welcome;

fn app() -> App {
    let metrics = Theme::<Rgb888>::light().metrics();
    App::new(metrics, Rect::new(0, 0, 960, 540)).unwrap()
}

/// Presses and releases each key; returns the buttons that were pressed.
fn keys(app: &mut App, keys: &[KeyCode]) -> Vec<&'static str> {
    let mut pressed = Vec::new();
    for &key in keys {
        pressed.extend(app.handle(InputEvent::KeyDown(key)).unwrap());
        pressed.extend(app.handle(InputEvent::KeyUp(key)).unwrap());
    }
    pressed
}

fn letters(text: &str) -> Vec<KeyCode> {
    text.chars()
        .map(|ch| KeyCode::letter(ch).unwrap())
        .collect()
}

#[test]
fn a_name_typed_is_greeted_on_the_next_screen() {
    let mut app = app();
    assert_eq!(app.screen(), "welcome");
    // Tab to the name, type it, Tab to Continue, Enter.
    keys(&mut app, &[KeyCode::TAB]);
    keys(&mut app, &letters("ada"));
    keys(&mut app, &[KeyCode::TAB]);
    assert_eq!(keys(&mut app, &[KeyCode::ENTER]), ["Continue"]);
    assert_eq!(app.screen(), "hello");
    assert_eq!(app.tree().slot_text(hello::slots::NAME), Some("ada"));
}

#[test]
fn continue_with_no_name_asks_for_one_and_stays() {
    let mut app = app();
    assert_eq!(app.tree().visible(welcome::groups::NAME_ERROR), Some(false));
    keys(&mut app, &[KeyCode::TAB, KeyCode::TAB]);
    assert_eq!(keys(&mut app, &[KeyCode::ENTER]), ["Continue"]);
    assert_eq!(app.screen(), "welcome");
    assert_eq!(app.tree().visible(welcome::groups::NAME_ERROR), Some(true));
    assert_eq!(
        app.tree().slot_text(welcome::slots::NAME_ERROR),
        Some("Please tell us your name")
    );
}

#[test]
fn start_again_goes_back_to_an_empty_form() {
    let mut app = app();
    keys(&mut app, &[KeyCode::TAB]);
    keys(&mut app, &letters("ada"));
    keys(&mut app, &[KeyCode::TAB, KeyCode::ENTER]);
    assert_eq!(app.screen(), "hello");
    // The only button has the focus after a Tab.
    keys(&mut app, &[KeyCode::TAB]);
    assert_eq!(keys(&mut app, &[KeyCode::ENTER]), ["Start again"]);
    assert_eq!(app.screen(), "welcome");
    assert_eq!(app.tree().value(welcome::fields::NAME), Some(""));
}

#[test]
fn a_screen_no_file_names_is_refused() {
    let mut app = app();
    assert!(app.show("nowhere").is_err());
    assert_eq!(app.screen(), "welcome");
}
