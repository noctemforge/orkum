mod util;

use orkum::AppWindow;
use slint::{ComponentHandle, PlatformError, SharedString};
use std::{cell::RefCell, rc::Rc};

use crate::util::state::AppState;

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    let state = Rc::new(RefCell::new(AppState::new()));

    let st_ref = state.clone();
    let ui_handle = ui.as_weak().clone();
    ui.on_open_file_clicked(move || {
        st_ref.borrow_mut().open_new_file(&ui_handle);
        st_ref.borrow().sync_file_state(ui_handle.unwrap());
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
        st_ref.borrow().sync_file_state(ui_handle.unwrap());
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

    ui.run()
}
