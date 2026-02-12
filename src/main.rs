// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs_util;

use slint::{ComponentHandle, PlatformError, VecModel, Weak};
use std::rc::Rc;

slint::include_modules!();

/// Top level app data references
#[derive(Clone)]
struct AppState {
    open_files: Rc<VecModel<FileEntry>>,
    main_window: Weak<AppWindow>,
}

fn main() -> Result<(), PlatformError> {
    let window = AppWindow::new()?;

    let files_model = Rc::new(VecModel::<FileEntry>::default());
    window.set_files(files_model.clone().into());

    let state = AppState {
        main_window: window.as_weak(),
        open_files: files_model.clone(),
    };

    window.on_open_file_clicked(move || fs_util::append_new_file(&state));

    window.run()
}
