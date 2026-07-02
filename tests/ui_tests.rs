#[cfg(test)]
mod ui_tests {
    use anyhow::Result;
    use orkum::{
        AppWindow,
        util::{file_model::FileModel, state::AppState},
    };
    use slint::ComponentHandle;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    #[ignore = "manual"]
    fn test_loaded_file() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "abcdefghijklmnopqrstuvwxyz")?;
        let path = file.path().to_path_buf();

        let ui = AppWindow::new()?;

        let mut state = AppState::new();
        state.add_file(FileModel::read_utf8(path)?);
        state.sync_file_state(&ui.as_weak());
        state.setup_event_handlers(&ui);

        Ok(ui.run()?)
    }
}
