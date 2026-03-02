use slint::{Model, ModelRc};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::{AppState, FileEntry, HexRow};

pub fn open_new_file(state: &AppState) {
    let win = state.main_window.unwrap();
    let files_model = state.open_files.clone();
    if let Some(path) = show_file_dialog() {
        match File::open(&path) {
            Ok(f) => {
                let meta = f.metadata().unwrap();

                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let hex_model_handle = HexModel {
                    file: f,
                    file_size: meta.len(),
                };

                files_model.push(FileEntry {
                    name: name.into(),
                    content: ModelRc::new(hex_model_handle),
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

struct HexModel {
    file: std::fs::File,
    file_size: u64,
}


impl Model for HexModel {
    type Data = HexRow;

    fn row_count(&self) -> usize {
        (self.file_size as f64 / 16.0).ceil() as usize
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        let mut buffer = [0u8; 16];
        let pos = (row * 16) as u64;
        let offset_str = format!("{:08X}", pos);

        // Seek to the specific part of the large file
        let mut file = &self.file;
        file.seek(SeekFrom::Start(pos)).ok()?;
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            return None;
        }

        // Format the bytes
        let hex_str = buffer[..n]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let ascii_str = buffer[..n]
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect::<String>();

        Some(HexRow {
            offset: offset_str.into(),
            hex: hex_str.into(),
            ascii: ascii_str.into(),
        })
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &()
    }
}

// fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
//     let mut entries = fs::read_dir(path)?
//         .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     entries.sort();

//     Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
// }
