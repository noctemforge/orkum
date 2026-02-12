use slint::{Model, ModelRc, SharedString, VecModel};
use std::io;
use std::path::PathBuf;
use std::{fs, rc::Rc};

use crate::{AppState, FileEntry};

pub fn append_new_file(state: &AppState) {
    let win = state.main_window.unwrap();
    let files_model = state.open_files.clone();
    if let Some(path) = show_file_dialog() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                files_model.push(FileEntry {
                    name: name.into(),
                    content: content.into(),
                });

                // Switch to the newly opened tab
                win.set_active_tab((files_model.row_count() - 1) as i32);
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }
}

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

fn show_dir_dialog(manifest: &PathBuf) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a directory")
        .set_directory(manifest.as_path())
        .pick_folder()
}

fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
}
