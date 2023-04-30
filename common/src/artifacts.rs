use crate::types::errors::Error;
use log::info;
use std::fs::File;
use std::io::Write;

pub fn save_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    info!("Stored {}", path);
    Ok(())
}
