/// Download specification containing FTP URLs, checksums, and local paths.
pub struct DownloadSpec {
    pub fastq_ftps: Vec<String>,
    pub fastq_md5s: Vec<String>,
    pub fastq_locals: Vec<String>,
}

impl DownloadSpec {
    pub fn current_length(&self) -> usize {
        let length = self.fastq_ftps.len();

        if length == self.fastq_md5s.len() && length == self.fastq_locals.len() {
            return length;
        }

        panic!("Unequal number of ftps, md5s and local path.")
    }
}

impl Iterator for DownloadSpec {
    type Item = (String, String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_length() == 0 {
            return None;
        }

        let ftp = self.fastq_ftps.remove(0);
        let md5 = self.fastq_md5s.remove(0);
        let local = self.fastq_locals.remove(0);

        Some((ftp, md5, local))
    }
}
