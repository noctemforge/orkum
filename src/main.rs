// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ComponentHandle, ModelRc, PlatformError, SharedString, VecModel, Weak};
use std::cell::RefCell;
use std::io;
use std::path::PathBuf;
use std::{fs::read_dir, rc::Rc};

slint::include_modules!();

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

    let state_copy = state.clone();
    window.on_set_new_dir(move || {
        let new_dir = show_open_dialog(&state_copy.borrow().working_dir);
        // FIX | TRY : Since we packed it, this should be safe
        let win = state_copy.borrow().main_window.unwrap();
        match list_dir(new_dir) {
            Ok(list) => win.set_dir_content(list),
            Err(e) => win.invoke_error_notification(SharedString::from(format!(
                "Could not open dir: {}",
                e
            ))),
        };
    });

    window.run()
}

fn show_open_dialog(manifest: &PathBuf) -> PathBuf {
    let dialog = rfd::FileDialog::new()
        .set_title("Select a manifest")
        .set_directory(manifest.as_path());

    dialog.pick_folder().unwrap_or_else(|| manifest.clone())
}

fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
}
