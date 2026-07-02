use crate::{AppWindow, FileEntry, util::file_model::FileModel};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};
use std::{cell::RefCell, rc::Rc};

/// Top level app data references
#[derive(Clone)]
pub struct AppState {
    active_file: Option<usize>,
    open_files: Vec<Rc<FileModel>>,
}

impl AppState {
    /// Creates an empty instance
    pub fn new() -> Self {
        AppState {
            open_files: Vec::new(),
            active_file: None,
        }
    }

    /// Consumes the instance ad couples the app state handling logic with the provided window
    pub fn setup_event_handlers(self, ui: &AppWindow) {
        let state = Rc::new(RefCell::new(self));

        let st_ref = state.clone();
        let ui_handle = ui.as_weak().clone();
        ui.on_open_file_clicked(move || {
            st_ref.borrow_mut().open_new_file(&ui_handle);
            st_ref.borrow().sync_file_state(&ui_handle);
        });

        let st_ref = state.clone();
        let ui_handle = ui.as_weak().clone();
        ui.on_switch_file(move |index| {
            st_ref.borrow_mut().set_active_index(index as usize);
            st_ref.borrow().load_active_file_hex(&ui_handle.unwrap());
        });

        let st_ref = state.clone();
        let ui_handle = ui.as_weak().clone();
        ui.on_close_file_clicked(move |index| {
            st_ref.borrow_mut().close_file(index);
            st_ref.borrow().sync_file_state(&ui_handle);
        });

        let st_ref = state.clone();
        let ui_handle = ui.as_weak().clone();
        ui.on_byte_edited(move |row, c, val| {
            match st_ref.borrow().get_active_file() {
                Some(active_file) => {
                    let abs_offset = (row as u64 * 16) + c as u64;
                    active_file
                        .pending_changes
                        .borrow_mut()
                        .insert(abs_offset, val.to_string());
                    active_file.notify.row_changed(row as usize);
                }
                None => ui_handle
                    .unwrap()
                    .invoke_error_notification(SharedString::from("no active file")),
            };
        });
    }

    /// Set the active file index
    fn set_active_index(&mut self, index: impl Into<Option<usize>>) {
        self.active_file = index.into();
    }

    /// Returns the active file model
    fn get_active_file(&self) -> Option<Rc<FileModel>> {
        if let Some(idx) = self.active_file {
            match self.open_files.get(idx) {
                Some(file) => return Some(file.clone()),
                None => return None,
            }
        }
        None
    }

    /// Open a new file dialog and loads the selected file
    fn open_new_file(&mut self, window: &Weak<AppWindow>) {
        match FileModel::new_utf8_dialog() {
            // RE: hardcoded encoding
            Ok(obj) => {
                if let Some(file) = obj {
                    self.add_file(file);
                }
            }
            Err(err) => window.unwrap().invoke_error_notification(
                format!("Could not set up file reader : {}", err).into(),
            ),
        }
    }

    ///Appends a new file to the open files list and switches to it
    pub fn add_file(&mut self, file: FileModel) {
        self.open_files.push(Rc::new(file));
        self.active_file = Some(self.open_files.len() - 1);
    }

    /// Loads the currently active file data into the app window
    fn close_file(&mut self, index: i32) {
        let idx = index as usize;
        self.open_files.remove(idx);
        if let Some(active_idx) = self.active_file
            && active_idx != idx
        {
            return;
        }
        self.active_file = if idx > 0 {
            Some(idx - 1)
        } else if self.open_files.len() > 0 {
            Some(0)
        } else {
            None
        };
    }

    /// Loads the currently active file data into the app window
    fn load_active_file_hex(&self, app_window: &AppWindow) {
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

    /// Updates app window with the current state data
    pub fn sync_file_state(&self, ui: &Weak<AppWindow>) {
        let app_window = ui.unwrap();
        let entry_list: VecModel<FileEntry> = self
            .open_files
            .iter()
            .map_while(|f| {
                let name = match f.reader.path().file_name() {
                    Some(name) => name.to_string_lossy().to_string().into(),
                    None => {
                        app_window
                            .invoke_error_notification(SharedString::from("unsupported file name"));
                        return None;
                    }
                };
                Some(FileEntry {
                    modified: !f.pending_changes.borrow().is_empty(),
                    name,
                })
            })
            .collect();
        app_window.set_open_files(ModelRc::new(entry_list));
        self.load_active_file_hex(&app_window);
    }
}
