use super::{PdfDocument, Result};
use std::path::Path;

pub struct PdfLoader;

impl PdfLoader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<PdfDocument> {
        PdfDocument::open(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pdf() {
        assert!(PdfLoader::is_pdf("test.pdf"));
        assert!(PdfLoader::is_pdf("test.PDF"));
        assert!(PdfLoader::is_pdf("test.Pdf"));
        assert!(!PdfLoader::is_pdf("test.txt"));
        assert!(!PdfLoader::is_pdf("test"));
    }
}
