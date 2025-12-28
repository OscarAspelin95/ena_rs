//! ENA FASTQ download library.
//!
//! This library provides functionality to download FASTQ files from the European Nucleotide Archive (ENA).
//! It handles fetching metadata, validating downloads, and managing single-end or paired-end data from Illumina, IonTorrent, PacBio and Oxford Nanopore.

pub mod download;
pub mod download_utils;
pub mod ena;
pub mod errors;
pub mod schemas;

pub use errors::AppError;
