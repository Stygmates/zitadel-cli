use std::fs;

use crate::env::config_file_path;

/// Logs out the user by deleting the file specified in the `CONFIG_FILE_PATH` environment variable
pub(crate) fn logout() -> std::io::Result<()> {
    let config_file = config_file_path();
    fs::remove_file(config_file)?;
    Ok(())
}
