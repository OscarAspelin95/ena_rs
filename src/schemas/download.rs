/// Download specification containing FTP URLs, checksums, and local paths.
///
/// All vectors have the same length, with corresponding indices for each file.
pub struct DownloadSpec {
    /// FTP URLs for FASTQ files
    pub fastq_ftps: Vec<String>,
    /// MD5 checksums for validation
    pub fastq_md5s: Vec<String>,
    /// Local file paths where files will be saved
    pub fastq_locals: Vec<String>,
}
