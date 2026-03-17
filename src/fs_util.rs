use slint::ModelNotify;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, fs};

use crate::{AppState, HexModel};

pub fn open_new_file(state: &mut AppState) {
    if let Some(path) = show_file_dialog() {
        if let Ok(meta) = fs::metadata(&path) {
            let new_file_handle = Rc::new(HexModel {
                path,
                file_size: meta.len(),
                pending_changes: RefCell::new(HashMap::new()),
                notify: ModelNotify::default(),
            });
            state.open_files.push(new_file_handle);
            state.active_file = Some(state.open_files.len() - 1);
        }

        // match File::open(&path) {
        //     Ok(f) => {
        //         let name = path
        //             .file_name()
        //             .unwrap_or_default()
        //             .to_string_lossy()
        //             .to_string();

        //         files_model.push(FileEntry {
        //             name: name.into(),
        //             content: ModelRc::new(hex_model_handle),
        //         });

        //         // Switch to the newly opened tab
        //         win.set_active_tab((files_model.row_count() - 1) as i32);
        //     }
        //     Err(e) => eprintln!("Error reading file: {}", e),
        // }
    }
}

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

// fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
//     let mut entries = fs::read_dir(path)?
//         .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     entries.sort();

//     Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
// }
