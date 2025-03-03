use std::{collections::{HashMap, HashSet}, sync::Arc};
use crate::{color::Color, destination::{CustomDestination, Destination}, err::LoggerError, level::Level, log_writer::{file::FileWriter, stderr::StdErrWriter, stdout::StdOutWriter, LogWriter}, write_options::WriteOptions, Logger};

pub struct LoggerBuilder {
    pub (crate) config: HashMap<Level, HashSet<(Destination, WriteOptions)>>,
    pub (crate) colors: HashMap<Level, Color>,
}

impl LoggerBuilder {

    pub fn add_destination(
        mut self,
        level: Level,
        destination: Destination,
        options: Option<WriteOptions>,
    ) -> Self {

        let destination = Self::normalize_destination(destination);
        
        let destinations = self.config.entry(level).or_insert_with(HashSet::new);
        
        let options = match options {
            Some(options) => options,
            None => WriteOptions::default(),
        };

        destinations.insert((destination, options));
        self
    }

    pub fn with_stdout(self, level: Level, options: Option<WriteOptions>) -> Self {
        self.add_destination(level, Destination::StdOut, options)
    }

    pub fn with_stderr(self, level: Level, options: Option<WriteOptions>) -> Self {
        self.add_destination(level, Destination::StdErr, options)
    }

    pub fn with_file(self, level: Level, path: &str, options: Option<WriteOptions>) -> Self {
        self.add_destination(level, Destination::File(path.to_string()), options)
    }

    pub fn with_custom(self, level: Level, destination: Box<dyn CustomDestination>, options: Option<WriteOptions>) -> Self {
        self.add_destination(level, Destination::Custom(destination), options)
    }

    pub fn with_color(mut self, level: Level, color: Color) -> Self {
        self.colors.insert(level, color);
        self
    }

    pub fn with_color_all(self, color: Color) -> Self {
        self
        .with_color(Level::Error, color)
        .with_color(Level::Warn, color)
        .with_color(Level::Info, color)
        .with_color(Level::Debug, color)
        .with_color(Level::Trace, color)
    }

    fn normalize_destination(destination: Destination) -> Destination {
        match destination {
            Destination::None => Destination::None,
            Destination::StdOut => Destination::StdOut,
            Destination::StdErr => Destination::StdErr,
            Destination::File(path) => Destination::File(FileWriter::normalize_path(&path)),
            Destination::Custom(custom) => custom.normalized()
        }
    }

    fn writer_from_destination(
        destination: &Destination, 
        output: &mut HashMap<Destination, Arc<dyn LogWriter + Send + Sync>>,
        writers: &mut Vec<Arc<dyn LogWriter + Send + Sync>>,
    ) -> Option<Arc<dyn LogWriter + Send + Sync>> {
        
        if let Some(writer) = output.get(destination) {
            return Some(Arc::clone(writer));
        }
        
        let writer: Arc<dyn LogWriter + Send + Sync> = match destination {
            Destination::None => return None,
            Destination::StdOut => Arc::new(StdOutWriter::new()),
            Destination::StdErr => Arc::new(StdErrWriter::new()),
            Destination::File(path) => match FileWriter::new(path) {
                Ok(writer) => Arc::new(writer),
                Err(err) => {
                    eprintln!("Failed to open file '{}': {}", path, err);
                    return None;
                }
            },
            Destination::Custom(custom) => custom.as_writer(),
        };
        
        output.insert(destination.clone(), Arc::clone(&writer));
        writers.push(Arc::clone(&writer));
        Some(writer)
    }

    pub fn build(self) -> Result<(), LoggerError> {
        
        let mut output: HashMap<Level, Vec<(Arc<dyn LogWriter + Send + Sync>, WriteOptions)>> = HashMap::new();
        let mut all_writers: Vec<Arc<dyn LogWriter + Send + Sync>> = Vec::new();
        let mut writers: HashMap<Destination, Arc<dyn LogWriter + Send + Sync>> = HashMap::new();
        let mut max_level = Level::Error;

        for (level, destinations) in &self.config {
            let entry = output.entry(*level).or_insert(Vec::new());
            for (destination, dest_opt) in destinations {
                if let Some(writer) = Self::writer_from_destination(destination, &mut writers, &mut all_writers) {
                    entry.push((
                        writer,
                        dest_opt.clone(),
                    ));
                }
            }

            if *level > max_level {
                max_level = *level;
            }
        }

        let logger = Box::new(Logger {
            output,
            writers: all_writers,
            max_level,
            colors: self.colors,
        });

        log::set_boxed_logger(logger).map_err(|err| LoggerError::BuildError(err.to_string()))?;
        log::set_max_level(max_level.into());

        Ok(())
    }
}