use slint::{Model, ModelNotify, VecModel};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

use crate::{ByteData, ByteState, RowData, util::file_reader::FileReader};

fn show_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a file")
        .set_directory(PathBuf::from("."))
        .pick_file()
}

pub struct FileModel {
    pub reader: Rc<FileReader>,
    pub pending_changes: Rc<RefCell<HashMap<u64, String>>>,
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

        let bytes: Vec<ByteData> = (0..n)
            .map(|i| {
                let abs_offset = (pos + i) as u64;
                if let Some(val) = changes.borrow().get(&abs_offset) {
                    ByteData {
                        value: val.into(),
                        state: match u8::from_str_radix(&val, 16) {
                            Ok(_) => ByteState::Modified,
                            Err(_) => ByteState::Error,
                        },
                    }
                } else if i < n {
                    ByteData {
                        value: format!("{:02X}", buffer[i]).into(),
                        state: ByteState::Synced,
                    }
                } else {
                    ByteData {
                        value: "  ".into(),
                        state: ByteState::Synced,
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
                    .unwrap_or(format!("{}", buffer[i]));
                // if b.is_ascii_graphic() || b == b' ' {
                //     b as char
                // } else {
                //     '.'
                // }
                b
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
