pub mod stdout;
pub mod stderr;
pub mod file;

pub trait LogWriter: Send + Sync {
    fn write(&self, data: &str) -> std::io::Result<()>;
    fn flush(&self) -> std::io::Result<()>;
}