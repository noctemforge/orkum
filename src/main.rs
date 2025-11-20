// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_util;

use slint::{ComponentHandle, PlatformError, Weak};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

slint::include_modules!();

/// Top level app data
struct AppState {
    working_dir: PathBuf,
    main_window: Weak<AppWindow>,
}

fn main() -> Result<(), PlatformError> {
    let window = AppWindow::new()?;

    let state = Rc::new(RefCell::new(AppState {
        working_dir: PathBuf::from("."),
        main_window: window.as_weak(),
    }));

    // FIX | TRY : Since we packed it, this should be safe
    window.on_update_dir(move || fs_util::update_working_dir(&state.clone().borrow()));

    window.run()
}
