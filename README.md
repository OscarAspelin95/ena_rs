# ena_rs
Cli for downloading FASTQ files from ENA.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.90.0

## Install
The easiest way to get started is to download a precompiled Linux binary from the latest [release](https://github.com/OscarAspelin95/ena_rs/releases).

## Install from source
Clone the repository or download the source code. Enter the ena_rs directory and run:<br>

`cargo build --release`

The generated binary is available in `target/release/ena_rs`.

## Usage
`ena_rs --accession <accession> --outdir <outdir>`

Required arguments:
<pre>
<b>--accession</b> A ENA project, sample or run accession.
<b>--outdir</b> Output directory.
</pre>


## Roadmap
- Add support for providing multiple accessions.
- Add filtering options for e.g., platform, fastq byte size, etc.
- Streaming support for large files.
- Add dynamic timeout based on fastq byte size.
