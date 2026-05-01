use slint::{Model, ModelNotify, VecModel};
use std::{
    collections::HashMap,
    fs::{metadata, File},
    io::SeekFrom,
    path::PathBuf,
    rc::Rc,
};

use crate::{AppState, ByteData, RowData};
use std::io::{Read, Seek};

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

impl AppState {
    pub fn open_new_file(&mut self) {
        if let Some(path) = show_file_dialog() {
            if let Ok(meta) = metadata(&path) {
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

    pub fn close_file(&mut self, index: i32) {
        self.open_files.remove(index as usize);
        self.active_file = if index > 0 {
            Some((index - 1) as usize)
        } else {
            None
        }
    }
}

pub struct FileModel {
    pub path: PathBuf,
    pub file_size: u64,
    pub pending_changes: HashMap<u64, u8>,
    pub notify: ModelNotify,
}

impl Model for FileModel {
    type Data = RowData;

    fn row_count(&self) -> usize {
        (self.file_size as f64 / 16.0).ceil() as usize
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        let pos = (row * 16) as u64;
        let mut buffer = [0u8; 16];

        let mut file = File::open(&self.path).ok()?;
        file.seek(SeekFrom::Start(pos)).ok()?;
        let n = file.read(&mut buffer).ok()?;
        if n == 0 && self.file_size > 0 {
            return None;
        }

        let changes = self.pending_changes.clone(); // RE: clone

        let bytes: Vec<ByteData> = (0..16)
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

        let ascii: String = (0..n)
            .map(|i| {
                let b = changes.get(&(pos + i as u64)).cloned().unwrap_or(buffer[i]);
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        Some(RowData {
            offset: format!("{:08X}", pos).into(),
            bytes: Rc::new(VecModel::from(bytes)).into(),
            ascii: ascii.into(),
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
