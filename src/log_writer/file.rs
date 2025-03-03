use std::{fs::{File, OpenOptions}, io::Write, path::PathBuf};
use parking_lot::Mutex;
use super::LogWriter;

pub struct FileWriter {
    file: Mutex<File>,
}

impl FileWriter {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self { file: Mutex::new(file) })
    }

    pub (crate) fn normalize_path(path: &str) -> String {
        let path_buf = PathBuf::from(path);
        if let Ok(canonical) = path_buf.canonicalize() {
            if let Some(path_str) = canonical.to_str() {
                return path_str.to_string();
            }
        }
        path.to_string()
    }
}

impl LogWriter for FileWriter {
    fn write(&self, data: &str) -> std::io::Result<()> {
        self.file.lock().write_all(data.as_bytes())
    }

    fn flush(&self) -> std::io::Result<()> {
        self.file.lock().flush()
    }
}