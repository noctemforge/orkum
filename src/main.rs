use orkum::{AppWindow, util::state::AppState};
use slint::{ComponentHandle, PlatformError};

fn main() -> Result<(), PlatformError> {
    let ui = AppWindow::new()?;

    AppState::new().setup_event_handlers(&ui);

    ui.run()
}
