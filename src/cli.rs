use clap::{Parser, ValueHint};
use std::{num::NonZeroUsize, path::PathBuf, time::Duration};


#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "High-performance REST API benchmark CLI")]
pub struct Cli {
  
    pub url: String,

  
    #[arg(short, long, default_value = "GET")]
    pub method: String,


    #[arg(
        short = 'H',
        long  = "header",
        value_parser = parse_header,
        number_of_values = 1
    )]
    pub headers: Vec<(String, String)>,


    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub body_file: Option<PathBuf>,

  
    #[arg(short, long, default_value = "10")]
    pub concurrency: NonZeroUsize,


    #[arg(short, long, group = "load")]
    pub requests: Option<NonZeroUsize>,

   
    #[arg(short, long, value_parser = humantime::parse_duration, group = "load")]
    pub duration: Option<Duration>,


    #[arg(long, default_value = "10s", value_parser = humantime::parse_duration)]
    pub timeout: Duration,

  
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub output: PathBuf,

    
    #[arg(short = 'f', long, default_value = "json")]
    pub output_format: String,
}


fn parse_header(raw: &str) -> Result<(String, String), String> {
    let (k, v) = raw
        .split_once(':')
        .ok_or("Header must be 'Key: Value'")?;
    Ok((k.trim().into(), v.trim().into()))
}
