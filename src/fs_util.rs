use slint::{Model, ModelNotify, ModelRc, VecModel, Weak};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use crate::{AppState, AppWindow, ByteData, RowData, file_reader::FileReader};

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

impl AppState {
    pub fn open_new_file(&mut self, window: &Weak<AppWindow>) {
        if let Some(path) = show_file_dialog() {
            match FileReader::new(path, encoding_rs::UTF_8) {
                // RE: hardcoded encoding
                Ok(obj) => {
                    let new_file_handle = Rc::new(FileModel {
                        reader: Rc::new(obj),
                        pending_changes: Rc::new(RefCell::new(HashMap::new())),
                        notify: ModelNotify::default(),
                    });
                    self.open_files.push(new_file_handle);
                    self.active_file = Some(self.open_files.len() - 1);
                }
                Err(err) => window.unwrap().invoke_error_notification(
                    format!("Could not set up file reader : {}", err).into(),
                ),
            }
        }
    }

    pub fn close_file(&mut self, index: i32) {
        self.open_files.remove(index as usize);
        if let Some(active_idx) = self.active_file
            && active_idx != index as usize
        {
            return;
        }
        self.active_file = if index > 0 {
            Some((index - 1) as usize)
        } else if self.open_files.len() > 0 {
            Some(0)
        } else {
            None
        };
    }

    pub fn load_active_file_hex(&self, app_window: &AppWindow) {
        if self.open_files.len() < 1 {
            return; // Guard double unwrap
        }
        let file = self
            .open_files
            .get(self.active_file.unwrap())
            .unwrap()
            .clone();
        app_window.set_hex_rows(ModelRc::new(file));
        if let Some(idx) = self.active_file {
            app_window.set_active_tab(idx as i32);
        }
    }
}

pub struct FileModel {
    pub reader: Rc<FileReader>,
    pub pending_changes: Rc<RefCell<HashMap<u64, u8>>>,
    pub notify: ModelNotify,
}

impl Model for FileModel {
    type Data = RowData;

    fn row_count(&self) -> usize {
        (self.reader.len() as f64 / 16.0).ceil() as usize
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        let pos = row * 16;
        let buffer = self.reader.next_16_bytes(pos);
        let n = buffer.len();

        let changes = self.pending_changes.clone();

        let bytes: Vec<ByteData> = (0..16)
            .map(|i| {
                let abs_offset = (pos + i) as u64;
                if let Some(&m_byte) = changes.borrow().get(&abs_offset) {
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
                let b = changes
                    .borrow()
                    .get(&((pos + i) as u64))
                    .cloned()
                    .unwrap_or(buffer[i]);
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
