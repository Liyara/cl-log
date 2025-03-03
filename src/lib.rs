use std::{collections::HashMap, sync::Arc};

use builder::LoggerBuilder;
use chrono::Local;
use color::Color;
use colored::Colorize;
use level::Level;
use log::{Metadata, Record};
use log_writer::LogWriter;
use write_options::WriteOptions;

pub mod err;
pub mod level;
pub mod color;
pub mod write_options;
pub mod destination;
pub mod log_writer;
pub mod builder;

pub struct Logger {
    output: HashMap<Level, Vec<(Arc<dyn LogWriter + Send + Sync>, WriteOptions)>>,
    writers: Vec<Arc<dyn LogWriter + Send + Sync>>,
    max_level: Level,
    colors: HashMap<Level, Color>,
}

impl Logger {

    pub fn builder() -> LoggerBuilder {
        LoggerBuilder {
            config: HashMap::new(),
            colors: HashMap::new(),
        }
    }


    /*

        [timestamp] LEVEL: args (module; <file> #line; ThreadId(thread_id))
        backtrace

     */
    fn record_as_string(&self, record: &Record, options: WriteOptions) -> String {

        let mut formatted_str = String::new();

        // Timestamp
        if options.contains(WriteOptions::TIMESTAMP) {
            formatted_str.push_str(&format!("[{}] ", Local::now().format("%Y-%m-%d %H:%M:%S%.3f")));
        }

        // Level
        if options.contains(WriteOptions::LEVEL) {
            let level: Level = record.level().into();
            let mut level_str = format!("{}: ", record.level().to_string().to_ascii_uppercase());
            if options.contains(WriteOptions::LEVEL_COLOR) {
                let color = match self.colors.get(&level) {
                    Some(color) => color,
                    None => &Color::from(level)
                };
                level_str = level_str.truecolor(color.r, color.g, color.b).to_string();
            }
            formatted_str.push_str(&level_str);
        }

        // Args
        formatted_str.push_str(format!("{} ", record.args()).as_str());

        let extra: bool =
            options.contains(WriteOptions::MODULE) ||
            options.contains(WriteOptions::FILE) ||
            options.contains(WriteOptions::LINE) ||
            options.contains(WriteOptions::THREAD)
        ;

        if extra {
            formatted_str.push_str("(");

            let mut separator: bool = false;

            // Module
            if options.contains(WriteOptions::MODULE) {
                formatted_str.push_str(format!("{}", record.module_path().unwrap_or("")).as_str());
                separator = true;
            }

            // File
            if options.contains(WriteOptions::FILE) {
                if separator { formatted_str.push_str("; "); }
                formatted_str.push_str(format!("<{}>", record.file().unwrap_or("")).as_str());
                separator = true;
            }

            // Line
            if options.contains(WriteOptions::LINE) {
                if separator { formatted_str.push_str(" "); }
                formatted_str.push_str(format!("#{}", record.line().unwrap_or(0)).as_str());
                separator = true;
            }

            // Thread
            if options.contains(WriteOptions::THREAD) {
                if separator { formatted_str.push_str("; "); }
                formatted_str.push_str(format!("{:?}", std::thread::current().id()).as_str());
            }

            formatted_str.push_str(")");
        }

        // Backtrace
        formatted_str.push_str("\n");
        if options.contains(WriteOptions::BACKTRACE) {
            formatted_str.push_str(format!("{:?}\n", backtrace::Backtrace::new()).as_str());
        }
        
        formatted_str
        
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() as usize <= self.max_level as usize
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        let output = match self.output.get(&record.level().into()) {
            Some(output) => output,
            None => return,
        };

        for writer in output {
            match writer.0.write(&self.record_as_string(record, writer.1)) {
                Ok(_) => {},
                Err(e) => eprintln!("Error writing to log: {}", e),
            }
        }
    }

    fn flush(&self) {
        for writer in self.writers.iter() {
            match writer.flush() {
                Ok(_) => {},
                Err(e) => eprintln!("Error flushing log: {}", e),
            }
        }
    }
}