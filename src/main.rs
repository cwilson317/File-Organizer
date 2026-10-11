use std::env;
use std::env::current_exe;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

mod file_detector;

fn main() -> Result<(), io::Error> {
    let mut current_path: PathBuf = env::current_exe().unwrap();

    current_path.pop();

    create_folders(&current_path);

    let images_path: PathBuf = current_path.join("images");
    let documents_path: PathBuf = current_path.join("documents");
    let spreadsheets_path: PathBuf = current_path.join("spreadsheets");
    let presentations_path: PathBuf = current_path.join("presentations");
    let plain_text_path: PathBuf = current_path.join("plain_text");
    let data_path: PathBuf = current_path.join("data");
    let video_path: PathBuf = current_path.join("video");
    let audio_path: PathBuf = current_path.join("audio");
    let archives_path: PathBuf = current_path.join("archives");
    let runnables_path: PathBuf = current_path.join("runnables");
    let other_path: PathBuf = current_path.join("other");
    let folders_path: PathBuf = current_path.join("folders");

    println!("Current in the directory: {:?}", &current_path);
    println!("it contains the following:");

    let directory_contents = match fs::read_dir(&current_path) {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };

    let current_exe: Option<PathBuf> = env::current_exe().ok();

    for files in directory_contents {

        let files = match files {
            Ok(f) => f,
            Err(_) => continue,
        };

        let file_path: PathBuf = files.path();
        let file_name: &std::ffi::OsStr = match file_path.file_name() {
            Some(name) => name,
            None => continue,
        };

        if Some(&file_path) == current_exe.as_ref() {
            continue;
        }

        if file_path.is_dir() {
            if file_path == images_path
                || file_path == documents_path
                || file_path == spreadsheets_path
                || file_path == presentations_path
                || file_path == plain_text_path
                || file_path == data_path
                || file_path == video_path
                || file_path == audio_path
                || file_path == archives_path
                || file_path == runnables_path
                || file_path == other_path
                || file_path == folders_path
            {
                continue;
            } else {
                let _ = match fs::rename(&file_path, &folders_path.join(&file_name)) {
                    Ok(()) => (),
                    Err(e) => panic!("Error: {e}"),
                };
            }
        } else {
            match file_detector::FileType::find_file_type(&file_path) {
                file_detector::FileType::PDF
                | file_detector::FileType::DOC
                | file_detector::FileType::DOCX
                | file_detector::FileType::RTF
                | file_detector::FileType::MD => {
                    if let Err(error) = fs::rename(&file_path, &documents_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::MP3 => {
                    if let Err(error) = fs::rename(&file_path, &audio_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::MP4 => {
                    fs::rename(&file_path, &video_path.join(&file_name))?
                }

                file_detector::FileType::JPEG
                | file_detector::FileType::PNG
                | file_detector::FileType::GIF
                | file_detector::FileType::BMP
                | file_detector::FileType::HEIC => {
                    if let Err(error) = fs::rename(&file_path, &images_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::EXE => {
                    if let Err(error) = fs::rename(&file_path, &runnables_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::TXT
                | file_detector::FileType::CSV
                | file_detector::FileType::TSV => {
                    if let Err(error) = fs::rename(&file_path, &plain_text_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::JSON | file_detector::FileType::LOG => {
                    if let Err(error) = fs::rename(&file_path, &data_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::ARCHIVE => {
                    if let Err(error) = fs::rename(&file_path, &archives_path.join(&file_name)) {
                        continue;
                    }
                }

                file_detector::FileType::XLS | file_detector::FileType::XLSX => {
                    if let Err(error) = fs::rename(&file_path, &spreadsheets_path.join(&file_name))
                    {
                        continue;
                    }
                }

                file_detector::FileType::PPT | file_detector::FileType::PPTX => {
                    if let Err(error) = fs::rename(&file_path, &presentations_path.join(&file_name))
                    {
                        continue;
                    }
                }

                file_detector::FileType::OTHER => {
                    if let Err(error) = fs::rename(&file_path, &other_path.join(&file_name)) {
                        continue;
                    }
                }
                _ => {
                    if let Err(error) = fs::rename(&file_path, &other_path.join(&file_name)) {
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_folders(path: &PathBuf) {
    if !path.is_dir() {
        return;
    }

    if !(&path.join("images").exists()) {
        let _ = match fs::create_dir(&path.join("images")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("documents").exists()) {
        let _ = match fs::create_dir(&path.join("documents")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("spreadsheets").exists()) {
        let _ = match fs::create_dir(&path.join("spreadsheets")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("presentations").exists()) {
        let _ = match fs::create_dir(&path.join("presentations")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("plain_text").exists()) {
        let _ = match fs::create_dir(&path.join("plain_text")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("data").exists()) {
        let _ = match fs::create_dir(&path.join("data")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("video").exists()) {
        let _ = match fs::create_dir(&path.join("video")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("audio").exists()) {
        let _ = match fs::create_dir(&path.join("audio")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("archives").exists()) {
        let _ = match fs::create_dir(&path.join("archives")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("runnables").exists()) {
        let _ = match fs::create_dir(&path.join("runnables")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("other").exists()) {
        let _ = match fs::create_dir(&path.join("other")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }

    if !(&path.join("folders").exists()) {
        let _ = match fs::create_dir(&path.join("folders")) {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
    }
}