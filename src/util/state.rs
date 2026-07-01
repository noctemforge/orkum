use slint::{ModelRc, Weak};
use std::{rc::Rc, slice::Iter};

use crate::{AppWindow, util::file_model::FileModel};

/// Top level app data references
#[derive(Clone)]
pub(crate) struct AppState {
    active_file: Option<usize>,
    open_files: Vec<Rc<FileModel>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            open_files: Vec::new(),
            active_file: None,
        }
    }

    pub fn set_active_index(&mut self, index: impl Into<Option<usize>>) {
        self.active_file = index.into();
    }

    pub fn iter_open_files(&self) -> Iter<'_, Rc<FileModel>> {
        self.open_files.iter()
    }

    pub fn get_active_file(&self) -> Option<Rc<FileModel>> {
        if let Some(idx) = self.active_file {
            match self.open_files.get(idx) {
                Some(file) => return Some(file.clone()),
                None => return None,
            }
        }
        None
    }

    pub fn open_new_file(&mut self, window: &Weak<AppWindow>) {
        match FileModel::new_utf8_dialog() {
            // RE: hardcoded encoding
            Ok(obj) => {
                if let Some(file) = obj {
                    self.open_files.push(Rc::new(file));
                    self.active_file = Some(self.open_files.len() - 1);
                }
            }
            Err(err) => window.unwrap().invoke_error_notification(
                format!("Could not set up file reader : {}", err).into(),
            ),
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
