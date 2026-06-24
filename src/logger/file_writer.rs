use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use chrono::Local;

pub struct FileLogger {
    writer: BufWriter<File>,
    current_date: String,
    log_dir: PathBuf,
}

impl FileLogger {
    pub fn new(log_dir: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir_all(&log_dir)?;
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let path = log_dir.join(format!("app-{}.jsonl", current_date));
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            current_date,
            log_dir,
        })
    }

    pub fn write(&mut self, entry: &super::LogEntry) -> io::Result<()> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        if today != self.current_date {
            self.rotate()?;
        }

        let json = serde_json::to_string(entry)?;
        writeln!(self.writer, "{}", json)
    }

    fn rotate(&mut self) -> io::Result<()> {
        self.writer.flush()?;
        let path = self.log_dir.join(format!("app-{}.jsonl", self.current_date));
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        self.writer = BufWriter::new(file);
        self.current_date = Local::now().format("%Y-%m-%d").to_string();
        Ok(())
    }
}
