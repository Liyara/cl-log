use std::io::Write;
use parking_lot::Mutex;
use super::LogWriter;

pub struct StdOutWriter {
    out: Mutex<std::io::Stdout>,
}

impl StdOutWriter {

    pub fn new() -> Self {
        Self { out: Mutex::new(std::io::stdout()) }
    }
    
}

impl LogWriter for StdOutWriter {
    fn write(&self, data: &str) -> std::io::Result<()> {
        self.out.lock().write_all(data.as_bytes())
    }
    fn flush(&self) -> std::io::Result<()> {
        self.out.lock().flush()
    }
}