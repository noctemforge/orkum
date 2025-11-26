use slint::{ModelRc, SharedString, VecModel};
use std::io;
use std::path::PathBuf;
use std::{fs::read_dir, rc::Rc};

use crate::AppState;

pub fn update_working_dir(state: &AppState) {
    let new_dir = match show_dir_dialog(&state.working_dir) {
        Some(dir) => dir,
        None => return,
    };
    let win = state.main_window.unwrap();
    match list_dir(new_dir) {
        Ok(list) => win.set_dir_content(list),
        Err(e) => win.invoke_error_notification(SharedString::from(format!(
            "Could not open directory: {}",
            e
        ))),
    };
}

fn show_dir_dialog(manifest: &PathBuf) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a directory")
        .set_directory(manifest.as_path())
        .pick_folder()
}

fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
}
