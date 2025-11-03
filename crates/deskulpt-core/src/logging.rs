use anyhow::Result;
use tauri::{App, Runtime};

use crate::states::LoggingStateExt;

/// Legacy entry point that delegates to [`LoggingStateExt::manage_logging`].
pub fn init<R: Runtime>(app: &mut App<R>) -> Result<()> {
    app.manage_logging()
}
