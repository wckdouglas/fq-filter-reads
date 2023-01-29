pub mod cli;

use bio::io::fastq;
use bio::io::fastq::Record;
use flate2::read::GzDecoder;
use log::{debug, info};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn process_read(record: Record, id_list: &HashSet<String>) -> Result<u64, String> {
    let mut out_count = 0;
    let seq_id: String = record.id().to_string();
    if id_list.contains(&seq_id) {
        println!("{}", record);
        out_count += 1;
    }
    Ok(out_count)
}

/// Filter out reads with ID not in the given id list
///
/// # Arguments
/// * `fastq_file` - file path to the fastq file for filtering
/// * `id_list` - hash table storing the hashed read names that won't be filtered out
///
/// # Returns
/// - tuple (total read count, number of reads retained)
///
/// # Example
/// ```
/// use std::collections::HashSet;
/// use std::io::prelude::*;
/// use std::fs::File;
/// use flate2::GzBuilder;
/// use flate2::Compression;
/// use fq_filter_reads::filter_fq;
///
/// // mock data
/// let tmp_fq = "/tmp/test.fq.gz";
/// let f = File::create(tmp_fq).unwrap();
/// let mut gz = GzBuilder::new()
///     .filename("test.fq.gz")
///     .comment("test file")
///     .write(f, Compression::default());
/// let data = b"@a\nAAA\n+\nAAA\n@c\nCCC\n+\nCCC\n@t\nTTT\n+\nTTT";
/// gz.write_all(data).unwrap();
/// gz.finish().unwrap();
///
/// let mut hash: HashSet<String> = HashSet::new();
/// hash.insert("a".to_string());
/// hash.insert("t".to_string());
///
/// let (read_count, out_count) = filter_fq(tmp_fq, &hash).unwrap();
/// assert_eq!(read_count, 3);
/// assert_eq!(out_count, 2);
/// ```
pub fn filter_fq(fastq_file: &str, id_list: &HashSet<String>) -> Result<(u64, u64), String> {
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

    Ok((read_count, out_count))
}

/// Getting fastq record ID from a file
///
/// # Arguments
/// * `id_file` - file path to the id list
///
/// # Returns
/// * `id_list` - the hash table for all ids
///
/// # Example
/// ```
/// use std::fs;
/// use fq_filter_reads::get_list;
/// let data = "a\nb\nc";
/// let filename = "/tmp/id_file";
/// fs::write(filename, data).unwrap();
///
/// let ids = get_list(filename).unwrap();
///
/// assert!(ids.contains(&"a".to_string()));
/// assert!(ids.contains(&"b".to_string()));
/// assert!(ids.contains(&"c".to_string()));
/// assert!(!ids.contains(&"d".to_string()));
///
/// ```
pub fn get_list(id_file: &str) -> Result<HashSet<String>, String> {
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
