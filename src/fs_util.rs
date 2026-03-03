use slint::{Model, ModelNotify, ModelRc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::rc::Rc;

use crate::{AppState, ByteData, FileEntry, HexRow};

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
                    pending_changes: RefCell::new(HashMap::new()),
                    notify: ModelNotify::default(),
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
    pending_changes: RefCell<HashMap<u64, u8>>,
    notify: ModelNotify,
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

        // let hex: Vec<ByteData> = buffer
        //     .iter()
        //     .enumerate()
        //     .map(|(i, &b)| {
        //         ByteData {
        //             value: format!("{:02X}", b).into(),
        //             is_modified: false, // You can check a HashMap here for pending changes
        //         }
        //     })
        //     .collect();

        let changes = self.pending_changes.borrow();

        let hex: Vec<ByteData> = (0..16)
            .map(|i| {
                let abs_offset = pos + i as u64;
                if let Some(&m_byte) = changes.get(&abs_offset) {
                    ByteData {
                        value: format!("{:02X}", m_byte).into(),
                        is_modified: true,
                    }
                } else if i < n {
                    ByteData {
                        value: format!("{:02X}", buffer[i]).into(),
                        is_modified: false,
                    }
                } else {
                    ByteData {
                        value: "  ".into(),
                        is_modified: false,
                    }
                }
            })
            .collect();

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
            hex: Rc::new(slint::VecModel::from(hex)).into(),
            ascii: ascii_str.into(),
        })
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &self.notify
    }
}

// fn list_dir(path: PathBuf) -> io::Result<ModelRc<SharedString>> {
//     let mut entries = fs::read_dir(path)?
//         .map(|res| res.map(|e| SharedString::from(e.path().display().to_string())))
//         .collect::<Result<Vec<_>, io::Error>>()?;

//     entries.sort();

//     Ok(ModelRc::from(Rc::new(VecModel::from(entries))))
// }
