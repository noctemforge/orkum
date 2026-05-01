// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

mod fs_util;

use slint::{ComponentHandle, ModelRc, PlatformError, SharedString, VecModel};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::fs_util::FileModel;

/// Top level app data references
#[derive(Clone)]
struct AppState {
    active_file: Option<usize>,
    open_files: Vec<Rc<FileModel>>,
}

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    let state = Rc::new(RefCell::new(AppState {
        open_files: Vec::new(),
        active_file: None,
    }));

    let st_ref = state.clone();
    let ui_handle = ui.as_weak().clone();
    ui.on_open_file_clicked(move || {
        st_ref.borrow_mut().open_new_file();
        update_file_state(ui_handle.unwrap(), st_ref.borrow());
    });

    let st_ref = state.clone();
    let ui_handle = ui.as_weak().clone();
    ui.on_switch_file(move |index| {
        st_ref.borrow_mut().active_file = Some(index as usize);
        st_ref.borrow().load_active_file_hex(&ui_handle.unwrap());
    });

    let st_ref = state.clone();
    let ui_handle = ui.as_weak().clone();
    ui.on_close_file_clicked(move |index| {
        st_ref.borrow_mut().close_file(index);
        update_file_state(ui_handle.unwrap(), st_ref.borrow());
    });

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

fn update_file_state(app_window: AppWindow, st_ref: Ref<'_, AppState>) {
    let entry_list: VecModel<FileEntry> = st_ref
        .open_files
        .iter()
        .map_while(|f| {
            let name = match f.path.file_name() {
                Some(name) => name.to_string_lossy().to_string().into(),
                None => {
                    app_window
                        .invoke_error_notification(SharedString::from("unsupported file name"));
                    return None;
                }
            };
            Some(FileEntry {
                modified: !f.pending_changes.is_empty(),
                name,
            })
        })
        .collect();
    app_window.set_open_files(ModelRc::new(entry_list));
    st_ref.load_active_file_hex(&app_window);
}
