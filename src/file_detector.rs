use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

const PDF_FILE_SIGNATURE: &[u8] = &[b'%', b'P', b'D', b'F'];

const MP3_FILE_SIGNATURE1: &[u8] = &[0x49, 0x44, 0x33];
const MP3_FILE_SIGNATURE2: &[u8] = &[0xFF, 0xFA];
const MP3_FILE_SIGNATURE3: &[u8] = &[0xFF, 0xFB];
const MP3_FILE_SIGNATURE4: &[u8] = &[0xFF, 0xF2];
const MP3_FILE_SIGNATURE5: &[u8] = &[0xFF, 0xF3];
const MP3_FILE_SIGNATURE6: &[u8] = &[0xFF, 0xE2];
const MP3_FILE_SIGNATURE7: &[u8] = &[0xFF, 0xE3];

const MP4_FILE_SIGNATURE1: &[u8] = &[
    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D,
];
const MP4_FILE_SIGNATURE2: &[u8] = &[
    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x6D, 0x70, 0x34, 0x31,
];
const MP4_FILE_SIGNATURE3: &[u8] = &[
    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x6D, 0x70, 0x34, 0x32,
];
const MP4_FILE_SIGNATURE4: &[u8] = &[
    0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x61, 0x76, 0x63, 0x31,
];
const MP4_FILE_SIGNATURE5: &[u8] = &[
    0x00, 0x00, 0x00, 0x14, 0x66, 0x74, 0x79, 0x70, 0x64, 0x61, 0x73, 0x68,
];
const MP4_FILE_SIGNATURE6: &[u8] = &[
    0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70, 0x4D, 0x53, 0x4E, 0x56,
];
const MP4_FILE_SIGNATURE7: &[u8] = &[
    0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x32,
];

const JPEG_FILE_SIGNATURE: &[u8] = &[0xFF, 0xD8];

const PNG_FILE_SIGNATURE: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

const GIF_FILE_SIGNATURE1: &[u8] = &[0x47, 0x49, 0x46, 0x38];
const GIF_FILE_SIGNATURE2: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x37, 0x61];
const GIF_FILE_SIGNATURE3: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61];

const BITMAP_FILE_SIGNATURE: &[u8] = &[0x42, 0x4D];

const HEIC_FILE_SIGNATURE: &[u8] = &[
    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x69, 0x63, 0x00, 0x00, 0x00, 0x00,
];

const RTF_FILE_SIGNATURE: &[u8] = &[0x7B, 0x5C, 0x72, 0x74, 0x66];

const EXE_FILE_SIGNATURE: &[u8] = &[0x4D, 0x5A];

const MSFT_LEGACY_FILE_SIGNATURE: &[u8] = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

const ARCHIVE_FILE_SIGNATURE1: &[u8] = &[0x50, 0x4B, 0x03, 0x04];
const ARCHIVE_FILE_SIGNATURE2: &[u8] = &[0x50, 0x4B, 0x05, 0x06];
const ARCHIVE_FILE_SIGNATURE3: &[u8] = &[0x50, 0x4B, 0x07, 0x08];
const ARCHIVE_FILE_SIGNATURE4: &[u8] = &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00];
const ARCHIVE_FILE_SIGNATURE5: &[u8] = &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00];
const ARCHIVE_FILE_SIGNATURE6: &[u8] = &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C];

#[derive(Debug)]
pub enum FileType {
    PDF,
    MP3,
    MP4,
    JPEG,
    PNG,
    GIF,
    BMP,
    HEIC,
    RTF,
    EXE,
    TXT,
    CSV,
    TSV,
    MD,
    JSON,
    LOG,
    ARCHIVE,
    DOC,
    XLS,
    PPT,
    DOCX,
    XLSX,
    PPTX,
    OTHER,
}

impl FileType {
    pub fn find_file_type(path: &PathBuf) -> FileType {
        if let Some(extension) = path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
            match extension.to_lowercase().as_str() {
                "txt" => return FileType::TXT,
                "csv" => return FileType::CSV,
                "tsv" => return FileType::TSV,
                "md" | "markdown" => return FileType::MD,
                "json" => return FileType::JSON,
                "log" => return FileType::LOG,
                _ => {}
            }
        }

        let file_bytes: [u8; 64] = get_file_first_bytes(path).unwrap_or([0; 64]);

        if file_bytes.starts_with(PDF_FILE_SIGNATURE) {
            return FileType::PDF;
        }

        if file_bytes.starts_with(MP3_FILE_SIGNATURE1)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE2)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE3)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE4)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE5)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE6)
            || file_bytes.starts_with(MP3_FILE_SIGNATURE7)
        {
            return FileType::MP3;
        }

        if file_bytes.starts_with(MP4_FILE_SIGNATURE1)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE2)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE3)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE4)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE5)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE6)
            || file_bytes.starts_with(MP4_FILE_SIGNATURE7)
        {
            return FileType::MP4;
        }

        if file_bytes.starts_with(JPEG_FILE_SIGNATURE) {
            return FileType::JPEG;
        }

        if file_bytes.starts_with(PNG_FILE_SIGNATURE) {
            return FileType::PNG;
        }

        if file_bytes.starts_with(GIF_FILE_SIGNATURE1)
            || file_bytes.starts_with(GIF_FILE_SIGNATURE2)
            || file_bytes.starts_with(GIF_FILE_SIGNATURE3)
        {
            return FileType::GIF;
        }

        if file_bytes.starts_with(BITMAP_FILE_SIGNATURE) {
            return FileType::BMP;
        }

        if file_bytes.starts_with(HEIC_FILE_SIGNATURE) {
            return FileType::HEIC;
        }

        if file_bytes.starts_with(MSFT_LEGACY_FILE_SIGNATURE) {
            if let Some(extension) = path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                match extension {
                    "doc" => return FileType::DOC,
                    "xls" => return FileType::XLS,
                    "ppt" => return FileType::PPT,
                    _ => {}
                }
            }
        }

        if file_bytes.starts_with(RTF_FILE_SIGNATURE) {
            return FileType::RTF;
        }

        if file_bytes.starts_with(EXE_FILE_SIGNATURE) {
            return FileType::EXE;
        }

        if file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE1)
            || file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE2)
            || file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE3)
            || file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE4)
            || file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE5)
            || file_bytes.starts_with(ARCHIVE_FILE_SIGNATURE6)
        {
            if let Some(extension) = path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                match extension {
                    "docx" => return FileType::DOCX,
                    "xlsx" => return FileType::XLSX,
                    "pptx" => return FileType::PPTX,
                    _ => return FileType::ARCHIVE,
                }
            }
        }

        FileType::OTHER
    }
}

fn get_file_first_bytes(path: &PathBuf) -> Result<[u8; 64], io::Error> {
    let mut file = File::open(path)?;
    let mut buffer: [u8; 64] = [0; 64];
    file.read(&mut buffer)?;
    Ok(buffer)
}
