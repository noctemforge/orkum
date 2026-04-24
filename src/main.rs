// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

mod fs_util;

use slint::{ComponentHandle, Model, ModelNotify, ModelRc, PlatformError, VecModel};
use std::io::{Read, Seek};
use std::{collections::HashMap, fs::File, io::SeekFrom, path::PathBuf, rc::Rc};

struct FileModel {
    path: PathBuf,
    file_size: u64,
    pending_changes: HashMap<u64, u8>,
    notify: ModelNotify,
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

        let changes = self.pending_changes.clone(); // TODO: reevaluate clone

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

/// Top level app data references
#[derive(Clone)]
struct AppState {
    // main_window: Weak<AppWindow>,
    active_file: Option<usize>,
    open_files: Vec<Rc<FileModel>>,
}

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    let mut state = AppState {
        open_files: Vec::new(),
        active_file: None,
    };

    let sync_ui = {
        let ui_weak = ui.as_weak();
        let st = state.clone();
        move || {
            let ui = ui_weak.unwrap();

            // 1. Update Tabs
            let tabs: Vec<FileEntry> = st
                .open_files
                .iter()
                .enumerate()
                .map(|(_, f)| {
                    let has_changes = !f.pending_changes.is_empty();
                    FileEntry {
                        name: f
                            .path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                            .into(),
                        modified: has_changes,
                    }
                })
                .collect();
            ui.set_open_files(Rc::new(VecModel::from(tabs)).into());

            // 2. Update Active File View
            if let Some(idx) = st.active_file {
                if let Some(active_file) = st.open_files.get(idx) {
                    ui.set_hex_rows(ModelRc::new(active_file.clone()));
                    ui.set_active_tab(idx as i32);
                }
            } else {
                ui.set_active_tab(-1);
            }
        }
    };

    ui.on_open_file_clicked(move || {
        state.open_new_file();
    });

    // let st_ui = state.clone();
    // let sync_switch = sync_ui.clone();
    // ui.on_switch_file(move |id| {
    //     let mut test = st_ui.borrow_mut();
    //     // test.active_file = Some(id as usize);
    //     sync_switch();
    // });
    // ui.on_close_file_clicked(move |index| {
    //     files_model.remove(index as usize);
    // });
    // window.on_byte_edited(move |f, r, c, val| {
    //     if (f < files_model.row_count()) {
    //         window.invoke_error_notification(SharedString::from("unloaded file edited"));
    //         return;
    //     }
    //     let m_edit = files_model.row_data(f).unwrap().content;
    //     // files_model[0];
    //     if val.len() == 2 {
    //         if let Ok(byte) = u8::from_str_radix(&val, 16) {
    //             let abs_offset = (r as u64 * 16) + c as u64;
    //             m_edit.pending_changes.borrow_mut().insert(abs_offset, byte);
    //             m_edit.notify.row_changed(r as usize);
    //         }
    //     }
    // });

    ui.run()
}
