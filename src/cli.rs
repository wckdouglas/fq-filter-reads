pub use clap::Parser;

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
