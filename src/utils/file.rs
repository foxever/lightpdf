use std::path::Path;

pub fn is_pdf_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

pub fn get_file_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

pub fn read_file_to_bytes<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    std::fs::read(path)
}
