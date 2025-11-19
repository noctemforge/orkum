// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::{fs::read_dir, rc::Rc};

use slint::{ModelRc, PlatformError, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let main_window = AppWindow::new()?;

    // slint::VecModel::from(list)

    match list_dir(".") {
        Ok(list) => main_window.set_dir_content(ModelRc::from(Rc::new(VecModel::from(list)))),
        Err(_) => todo!(),
    }

    main_window.run()
}

fn list_dir(path: &str) -> io::Result<Vec<SharedString>> {
    let mut entries = read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let out = entries
        .iter()
        .map(|f| SharedString::from(f.display().to_string()))
        .collect::<Vec<SharedString>>();

    Ok(out)
}
