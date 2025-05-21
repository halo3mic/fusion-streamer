use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use fusion_streamer::events::FusionEvent;

pub struct JsonlWriter {
    out_path: PathBuf,
}

impl JsonlWriter {
    pub fn new(out_path: impl Into<PathBuf>) -> Self {
        Self { out_path: out_path.into() }
    }

    pub fn write(&self, data: &FusionEvent) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.out_path)?;
        
        let json = serde_json::to_string(data)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }
}