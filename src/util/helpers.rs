use std::fs;
use std::io;
use std::path::Path;

pub fn remove_dir_files<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            panic!("ABORTED!! Subfolders detected");
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
