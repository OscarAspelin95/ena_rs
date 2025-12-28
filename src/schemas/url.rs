use crate::schemas::report::EnaFileReport;
use std::fmt::Display;
use struct_field_names_as_array::FieldNamesAsSlice;

pub enum EnaUrlType {
    Https,
}

impl Display for EnaUrlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaUrlType::Https => write!(f, "https://"),
        }
    }
}

pub enum EnaFormat {
    Json,
}

impl Display for EnaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnaFormat::Json => write!(f, "json"),
        }
    }
}

pub struct EnaUrl {
    base: String,
    fields: String,
    url_type: EnaUrlType,
    format: EnaFormat,
}

impl EnaUrl {
    pub fn new() -> Self {
        Self {
            base: "www.ebi.ac.uk/ena/portal/api/filereport".to_string(),
            fields: EnaFileReport::FIELD_NAMES_AS_SLICE.join(","),
            url_type: EnaUrlType::Https,
            format: EnaFormat::Json,
        }
    }

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
