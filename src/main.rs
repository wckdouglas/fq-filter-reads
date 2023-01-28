use bio::io::fastq;
use bio::io::fastq::Record;
use flate2::read::GzDecoder;
use log::{debug, info};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use clap::Parser;

/// Retaining fastq records with the given ID
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    /// input fastq file path  
    #[clap(long, value_parser)]
    pub in_fastq: String,

    ///  input id list file, one per line
    #[clap(long, value_parser)]
    pub in_id_list: String,
}

fn process_read(record: Record, id_list: &HashSet<String>) -> Result<u64, String> {
    let mut out_count = 0;
    let seq_id: String = record.id().to_string();
    if id_list.contains(&seq_id) {
        println!("{}", record);
        out_count += 1;
    }
    Ok(out_count)
}

fn filter_fq(fastq_file: &str, id_list: &HashSet<String>) -> Result<(), String> {
    let is_gz_input = fastq_file.ends_with(".gz");
    //let reader = fastq::Reader::from_file(fastq_file).map_err(|e| e.to_string())?;
    let mut read_count = 0;
    let mut out_count = 0;

    if is_gz_input {
        let reader = File::open(fastq_file)
            .map(BufReader::new)
            .map(GzDecoder::new)
            .map(fastq::Reader::new)
            .map_err(|e| e.to_string())?;

        for result in reader.records() {
            let record = result.map_err(|e| e.to_string())?;
            let oc = process_read(record, id_list)?;
            read_count += 1;
            out_count += oc
        }
    } else {
        return Err("Input must be gz fastq file".to_string());
    }

    info!(
        "Read {} alignments; Written {} records",
        read_count, out_count
    );
    Ok(())
}

fn get_list(id_file: &str) -> Result<HashSet<String>, String> {
    let file = File::open(id_file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut id_set = HashSet::new();
    for line in reader.lines() {
        let id = line.map_err(|e| e.to_string())?;
        debug!("{}", id);
        id_set.insert(id);
    }
    info!("Collected {} ids", id_set.len());
    Ok(id_set)
}

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
