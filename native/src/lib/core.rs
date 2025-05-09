use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    time::Instant,
};

use reqwest::Url;
use std::io::{Error, ErrorKind};

pub fn download_file(args: &crate::args::Args, url: Url) -> Result<(), Error> {
    use super::helpers::*;

    let mut response = reqwest::blocking::get(url.clone())
        .map_err(|_| Error::new(ErrorKind::ConnectionRefused, "Failed to connect to URL"))?;

    if !response.status().is_success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Server error: {}", response.status()),
        ));
    }

    cliclack::log::success(format!("Connection established ({})", response.status())).unwrap();

    let progress = cliclack::progress_bar(100);

    let total_size = response.content_length().unwrap_or(0);
    let headers = response.headers();
    let mut filename = resolvers::filename::attempt_all(args, &url, headers);
    let dest = std::env::current_dir()
        .map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to get current dir: {}", e),
            )
        })?
        .join(&filename);

    if dest.exists() {
        let filename_path = Path::new(&filename);
        let stem = filename_path
            .file_stem()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Invalid file name"))?
            .to_str()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Non-UTF8 file name"))?;

        let ext = filename_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unk");

        let new_filename = format!("{}_{}.{}", stem, random_string(8), ext);

        cliclack::log::remark(format!("File {} already exists", &filename)).unwrap();
        let selected_action = cliclack::select("What to do?")
            .item(FileExistAction::Continue, "Continue", &new_filename)
            .item(FileExistAction::Overwrite, "Overwrite", "")
            .item(FileExistAction::Cancel, "Cancel", "")
            .interact()
            .map_err(|e| Error::new(ErrorKind::Interrupted, format!("Input failed: {}", e)))?;

        match selected_action {
            FileExistAction::Continue => filename = new_filename,
            FileExistAction::Cancel => {
                cliclack::log::error("Operation was canceled").unwrap();
                std::process::exit(1);
            }
            _ => {}
        }
    }

    let mut dest_file = File::create(&filename).map_err(|e| {
        Error::new(
            ErrorKind::PermissionDenied,
            format!("Failed to create file: {}", e),
        )
    })?;

    let mut buffer = [0; 8192];
    let mut downloaded = FileSize::default();
    let mut speed = FileSize::default();

    let start_time = Instant::now();

    progress.start("Downloading...");

    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }

        dest_file
            .write_all(&buffer[..n])
            .map_err(|e| Error::new(ErrorKind::WriteZero, format!("Write failed: {}", e)))?;

        downloaded.bytes += n as u64;

        if total_size > 0 {
            let percent = (downloaded.bytes as f64 / total_size as f64 * 100.0).min(100.0);
            progress.set_position(percent as u64);
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        speed.bytes = if elapsed > 0.0 {
            (downloaded.bytes as f64 / elapsed) as u64
        } else {
            0
        };

        progress.set_message(format!(
            "{:.2} {}/s | {:.2} {}",
            speed.size(),
            speed.unit(),
            downloaded.size(),
            downloaded.unit()
        ));
    }

    progress.stop(format!(
        "{} ({} {})",
        filename,
        downloaded.size(),
        downloaded.unit()
    ));

    Ok(())
}
