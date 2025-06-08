use crate::stats::{Sample, Summary};
use csv::Writer;
use serde_json::to_writer_pretty;
use std::{
    error::Error,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};


pub trait OutputSink {
    fn write(&self, summary: &Summary, samples: &[Sample]) -> Result<(), Box<dyn Error>>;
}


pub struct JsonSink { path: PathBuf }
impl JsonSink { pub fn new(p: &Path) -> Result<Self, Box<dyn Error>> { Ok(Self { path: p.into() }) } }
impl OutputSink for JsonSink {
    fn write(&self, summary: &Summary, samples: &[Sample]) -> Result<(), Box<dyn Error>> {
        let f = BufWriter::new(File::create(&self.path)?);
        to_writer_pretty(f, &serde_json::json!({ "summary": summary, "samples": samples }))?;
        Ok(())
    }
}


pub struct CsvSink { base: PathBuf }
impl CsvSink { pub fn new(p: &Path) -> Result<Self, Box<dyn Error>> { Ok(Self { base: p.into() }) } }
impl OutputSink for CsvSink {
    fn write(&self, summary: &Summary, samples: &[Sample]) -> Result<(), Box<dyn Error>> {
        {
            let f = BufWriter::new(File::create(self.base.with_extension("summary.csv"))?);
            let mut w = Writer::from_writer(f);
            w.serialize(summary)?;
            w.flush()?;
        }

        {
            let f = BufWriter::new(File::create(self.base.with_extension("samples.csv"))?);
            let mut w = Writer::from_writer(f);
            for s in samples { w.serialize(s)?; }
            w.flush()?;
        }
        Ok(())
    }
}
