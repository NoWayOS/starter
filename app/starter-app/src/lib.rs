// SPDX-License-Identifier: MIT OR Apache-2.0
//! The application: two screens and what happens between them.
//!
//! The screens are written in TSX (`tsx/`) and converted to Rust by Screen
//! Compiler (`src/generated/`, never edited by hand). This file is the
//! logic: which screen is showing, and what each button does. It is the
//! same code on the device (the UI protection domain, `pds/ui`), in the
//! browser window while you work (`make dev`) and in the tests
//! (`tests/app.rs`).
//!
//! The pattern, for every screen you add:
//!
//! 1. a callback prop in the TSX (`onContinue`) is an **action** here
//!    (`welcome::actions::CONTINUE`): match on it in [`App::handle`];
//! 2. a control with an `id` that holds a value is a **field**: read it with
//!    `tree.value(welcome::fields::NAME)`;
//! 3. a string prop (`name`) is a **slot**: fill it with `tree.set_slot`;
//!    content inside `{props.x && ...}` is a **group**: show or hide it with
//!    `tree.set_visible`.
//!
//! No heap, no panics, no `unsafe`: the workspace's lints hold every line
//! to that, as NoWayOS's own code is held.

#![no_std]

pub mod generated;

use generated::hello::hello_screen as hello;
use generated::welcome::welcome_screen as welcome;
use generated::{CAPACITY, SCREENS, TEXT};
use nowayos_bounded::BoundedStr;
use nowayos_display::Rect;
use nowayos_input::InputEvent;
use nowayos_ui::{TextMetrics, Tree, UiError, UiEvent};

/// The screen the application starts on: its TSX file's name.
pub const FIRST: &str = "welcome";

/// The longest name kept, in bytes: the input's `maxLength` in characters,
/// with room for letters that take several bytes.
const NAME_BYTES: usize = 160;

/// The application: its tree, laid out for the display, and where it is.
pub struct App {
    tree: Tree<CAPACITY, TEXT>,
    /// The screen showing: its TSX file's name.
    showing: &'static str,
    metrics: TextMetrics,
    viewport: Rect,
}

impl App {
    /// The application on its first screen, laid out in `viewport` with
    /// `metrics`: the theme's font at the display's device scale.
    ///
    /// # Errors
    ///
    /// A [`UiError`] if the screen cannot be built or laid out.
    pub fn new(metrics: TextMetrics, viewport: Rect) -> Result<Self, UiError> {
        let mut app = Self {
            tree: Tree::new(),
            showing: FIRST,
            metrics,
            viewport,
        };
        app.show(FIRST)?;
        Ok(app)
    }

    /// The tree to draw.
    #[must_use]
    pub const fn tree(&self) -> &Tree<CAPACITY, TEXT> {
        &self.tree
    }

    /// The tree, to draw only what changed and then mark it drawn.
    pub const fn tree_mut(&mut self) -> &mut Tree<CAPACITY, TEXT> {
        &mut self.tree
    }

    /// The screen showing: its TSX file's name.
    #[must_use]
    pub const fn screen(&self) -> &'static str {
        self.showing
    }

    /// One input event. Returns the name of the button it pressed, if it
    /// pressed one - for the log, which never holds anything typed.
    ///
    /// # Errors
    ///
    /// A [`UiError`] if the next screen cannot be built.
    pub fn handle(&mut self, event: InputEvent) -> Result<Option<&'static str>, UiError> {
        let Some(UiEvent::Action(action)) = self.tree.handle(event) else {
            return Ok(None);
        };
        // Which screen, and which of its buttons.
        if self.showing == "welcome" && action == welcome::actions::CONTINUE {
            self.greet()?;
            Ok(Some("Continue"))
        } else if self.showing == "hello" && action == hello::actions::START_AGAIN {
            self.show("welcome")?;
            Ok(Some("Start again"))
        } else {
            Ok(None)
        }
    }

    /// Continue on the welcome screen: with a name, on to the greeting;
    /// without one, the screen says what it needs.
    fn greet(&mut self) -> Result<(), UiError> {
        // The name is copied out of the tree, because showing the next
        // screen clears the tree and everything typed into it.
        let name: BoundedStr<NAME_BYTES> =
            BoundedStr::from_str_truncated(self.tree.value(welcome::fields::NAME).unwrap_or(""));
        if name.as_str().trim().is_empty() {
            self.tree
                .set_slot(welcome::slots::NAME_ERROR, "Please tell us your name")?;
            self.tree.set_visible(welcome::groups::NAME_ERROR, true)?;
            return self.tree.layout(self.viewport, &self.metrics);
        }
        self.show("hello")?;
        self.tree
            .set_slot(hello::slots::NAME, name.as_str().trim())?;
        self.tree.layout(self.viewport, &self.metrics)
    }

    /// Replaces what is showing with the screen `name` - a TSX file's name -
    /// wiping whatever was typed.
    ///
    /// # Errors
    ///
    /// [`UiError::NoRoot`] for a name no screen has, or the tree's error if
    /// the screen does not fit it.
    pub fn show(&mut self, name: &str) -> Result<(), UiError> {
        let Some(&(screen, build)) = SCREENS.iter().find(|(screen, _)| *screen == name) else {
            return Err(UiError::NoRoot);
        };
        self.tree.clear();
        let root = build(&mut self.tree)?;
        self.tree.set_root(root)?;
        self.tree.layout(self.viewport, &self.metrics)?;
        self.showing = screen;
        Ok(())
    }
}
