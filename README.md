# fq-filter-reads

[![CI](https://github.com/wckdouglas/fq-filter-reads/actions/workflows/ci.yml/badge.svg)](https://github.com/wckdouglas/fq-filter-reads/actions/workflows/ci.yml)

This is a simple program to filter a fastq file with a provided list of IDs.

# Install

Installation from source:

```
git clone https://github.com/wckdouglas/fq-filter-reads.git
cd fq-filter-reads
cargo install --path .
```

# Usage 

```
$ fq-filter-reads -h
Retaining fastq records with the given ID

Usage: fq-filter-reads [OPTIONS] --in-fastq <IN_FASTQ> --in-id-list <IN_ID_LIST>

Options:
      --in-fastq <IN_FASTQ>      input fastq file path
      --in-id-list <IN_ID_LIST>  input id list file, one per line
      --inverse                  only keep reads with ids NOT in the given id list i.e. removing any records with id in the id list
  -h, --help                     Print help
  -V, --version                  Print version
```

# Example

Some test examples can be found under `test_script.sh`