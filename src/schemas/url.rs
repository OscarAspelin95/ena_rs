use crate::schemas::report::EnaFileReport;
use std::fmt::Display;
use struct_field_names_as_array::FieldNamesAsSlice;

/// URL protocol type for ENA API.
pub enum EnaUrlType {
    /// HTTPS protocol
    Https,
}

impl Display for EnaUrlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaUrlType::Https => write!(f, "https://"),
        }
    }
}

/// Response format for ENA API.
pub enum EnaFormat {
    /// JSON format
    Json,
}

impl Display for EnaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaFormat::Json => write!(f, "json"),
        }
    }
}

/// Builder for constructing ENA API query URLs.
pub struct EnaUrl {
    /// Base API endpoint
    base: String,
    /// Comma-separated field names to retrieve
    fields: String,
    /// Protocol type
    url_type: EnaUrlType,
    /// Response format
    format: EnaFormat,
}

impl Default for EnaUrl {
    fn default() -> Self {
        Self::new()
    }
}

impl EnaUrl {
    /// Creates a new ENA URL builder with default settings.
    pub fn new() -> Self {
        Self {
            base: "www.ebi.ac.uk/ena/portal/api/filereport".to_string(),
            fields: EnaFileReport::FIELD_NAMES_AS_SLICE.join(","),
            url_type: EnaUrlType::Https,
            format: EnaFormat::Json,
        }
    }

    /// Builds the complete URL for querying ENA metadata.
    ///
    /// # Arguments
    /// * `accession` - ENA accession number (run, sample, or study)
    ///
    /// # Returns
    /// Complete URL string for the API request
    pub fn build(&self, accession: &str) -> String {
        let url = format!(
            "{}{}?accession={}&result=read_run&fields={}&format={}",
            self.url_type, self.base, accession, self.fields, self.format
        );
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_build() {
        let url = EnaUrl::new();
        let built_url = url.build("SRR123456");

        assert!(built_url.starts_with("https://"));
        assert!(built_url.contains("www.ebi.ac.uk/ena/portal/api/filereport"));
        assert!(built_url.contains("accession=SRR123456"));
        assert!(built_url.contains("result=read_run"));
        assert!(built_url.contains("fields="));
        assert!(built_url.contains("format=json"));
    }

    #[test]
    fn test_url_includes_all_fields() {
        let url = EnaUrl::new();
        let built_url = url.build("TEST123");

        EnaFileReport::FIELD_NAMES_AS_SLICE
            .iter()
            .for_each(|field| {
                assert!(built_url.contains(field));
            });
    }
}
