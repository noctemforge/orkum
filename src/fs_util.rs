use slint::ModelNotify;
use std::{collections::HashMap, fs, path::PathBuf, rc::Rc};

use crate::{AppState, FileModel};

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

impl AppState {
    pub fn open_new_file(&mut self) {
        if let Some(path) = show_file_dialog() {
            if let Ok(meta) = fs::metadata(&path) {
                let new_file_handle = Rc::new(FileModel {
                    path,
                    file_size: meta.len(),
                    pending_changes: HashMap::new(),
                    notify: ModelNotify::default(),
                });
                self.open_files.push(new_file_handle);
                self.active_file = Some(self.open_files.len() - 1);
            }
        }
    }
}

// fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
//     let mut entries = fs::read_dir(path)?
//         .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     entries.sort();

//     Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
// }
