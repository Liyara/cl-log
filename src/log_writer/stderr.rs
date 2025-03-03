use std::io::Write;
use parking_lot::Mutex;
use super::LogWriter;

pub struct StdErrWriter {
    err: Mutex<std::io::Stderr>,
}

impl StdErrWriter {
    pub fn new() -> Self {
        Self { err: Mutex::new(std::io::stderr()) }
    }
}

impl LogWriter for StdErrWriter {
    fn write(&self, data: &str) -> std::io::Result<()> {
        self.err.lock().write_all(data.as_bytes())
    }
    fn flush(&self) -> std::io::Result<()> {
        self.err.lock().flush()
    }
}