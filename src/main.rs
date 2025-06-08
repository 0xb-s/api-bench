mod benchmark;
mod cli;
mod output;
mod stats;

use clap::Parser;
use cli::Cli;
use output::{CsvSink, JsonSink, OutputSink};
use std::error::Error;
use tokio::runtime::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()?;

    let args = Cli::parse();

    rt.block_on(async {
        let (summary, samples) = benchmark::run(&args).await?;

        let sink: Box<dyn OutputSink> = match args.output_format.as_str() {
            "csv" => Box::new(CsvSink::new(&args.output)?),
            _     => Box::new(JsonSink::new(&args.output)?),
        };
        sink.write(&summary, &samples)?;

        println!("?  Results written to {}", args.output.display());
        Ok::<(), Box<dyn Error>>(())
    })?;

    Ok(())
}
