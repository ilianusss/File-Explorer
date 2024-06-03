// COMPRESSION USING ZIP

use std::fs::{self, File};
use std::io::{self};
use std::path::Path;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;
use zip::read::ZipArchive;

use std::process::Command;

// Compress VIDEO
pub fn compress_video(video_input: &str, video_output: &str) {
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i").arg(video_input)
        .arg("-s").arg("1280x720")
        .arg("-r").arg("30")  // Increase frame rate to 30 fps
        .arg("-c:v").arg("libx264")
        .arg("-b:v").arg("600k")
        .arg("-b:a").arg("44100")
        .arg("-ac").arg("2")
        .arg("-ar").arg("22050")
        .arg("-tune").arg("fastdecode")
        .arg("-preset").arg("ultrafast")
        .arg(video_output)
        .status()
        .expect("Failed to execute ffmpeg");
}




// Compress Folder
pub fn compress_folder(folder_path: &str, output_path: &str) -> io::Result<()> {
    let folder = Path::new(folder_path);
    let output_file = fs::File::create(output_path)?;
    let mut zip_writer = ZipWriter::new(output_file);

    add_folder_contents_to_zip(folder, &mut zip_writer, folder)?;

    zip_writer.finish()?;
    Ok(())
}

fn add_folder_contents_to_zip(
    folder: &Path,
    zip_writer: &mut ZipWriter<fs::File>,
    base_folder: &Path,
) -> io::Result<()> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = match path.strip_prefix(base_folder) {
            Ok(rel_path) => rel_path,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to strip prefix",
                ));
            }
        };

        if path.is_file() {
            add_file_to_zip(&path, zip_writer, base_folder)?;
        } else if path.is_dir() {
            add_directory_to_zip(&relative_path, zip_writer)?;
            add_folder_contents_to_zip(&path, zip_writer, base_folder)?;
        }
    }

    Ok(())
}

fn add_file_to_zip(
    file_path: &Path,
    zip_writer: &mut ZipWriter<fs::File>,
    base_folder: &Path,
) -> io::Result<()> {
    let relative_path = file_path.strip_prefix(base_folder)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let file_name = relative_path.to_string_lossy().into_owned();
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);
    zip_writer.start_file(file_name, options)?;
    let mut file = fs::File::open(&file_path)?;
    io::copy(&mut file, zip_writer)?;
    Ok(())
}


fn add_directory_to_zip(
    dir_path: &Path,
    zip_writer: &mut ZipWriter<fs::File>,
) -> io::Result<()> {
    let dir_name = dir_path.to_string_lossy().into_owned();
    zip_writer.add_directory(dir_name, FileOptions::default())?;
    Ok(())
}


pub fn uncompress_folder(zip_file_path: &str, output_dir: &str) -> Result<(), String> {
    // Open the zip file
    let file = File::open(zip_file_path).map_err(|e| format!("Failed to open zip file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

    // Create the output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        std::fs::create_dir(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Extract each file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to read file in zip: {}", e))?;
        let outpath = Path::new(output_dir).join(file.name());

        if file.is_dir() {
            // Create directory if it doesn't exist
            std::fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create directory in zip: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| format!("Failed to create directory for file in zip: {}", e))?;
                }
            }

            // Extract file
            let mut outfile = File::create(&outpath).map_err(|e| format!("Failed to create output file in zip: {}", e))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| format!("Failed to extract file in zip: {}", e))?;
        }
    }

    Ok(())
}









