use clap::Parser;
use fq_filter_reads::cli::Command;
use fq_filter_reads::{filter_fq, get_list};

/// wrapper function to run the pipeline
fn run() -> Result<(), String> {
    let args = Command::parse();
    let id_set = get_list(&args.in_id_list)?;
    filter_fq(&args.in_fastq, &id_set)?;
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let result = run();
    match result {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    };
}
