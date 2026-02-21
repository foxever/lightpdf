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
    // Tests require actual PDF files
}
