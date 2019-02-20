use error::{Error, ErrorKind};
use std::path::Path;

/// Check if the provided path provides a valid file reference.
pub fn check_file_path<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    match path.as_ref().parent() {
        Some(p) => {
            if p.exists() {
                Ok(())
            } else {
                Err(ErrorKind::InvalidPath.into())
            }
        }
        None => Err(ErrorKind::InvalidPath.into()),
    }
}
