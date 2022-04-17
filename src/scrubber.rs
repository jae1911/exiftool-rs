use rexiv2::Metadata;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;

use log::{info, warn};
use walkdir::WalkDir;

mod utils;

pub fn scrub_image_file(
    image_path: &std::path::Path,
    keep_filename: bool,
) -> Result<(), Box<dyn Error>> {
    info!("> Found a path {}, processing!\n", image_path.display());
    info!("\n> Attempting to clean...\n");

    if let Ok(meta) = Metadata::new_from_path(image_path) {
        // EXIF
        if meta.supports_exif() {
            info!("> EXIF data found!\n");
            meta.clear_exif();
            info!("> Cleared all EXIF data!\n");
        }

        // XMP
        if meta.has_xmp() {
            info!("> XMP data found!\n");
            meta.clear_xmp();
            info!("> Cleared all XMP data!\n");
        }

        // IPTC
        if meta.has_iptc() {
            info!("> IPTC data found!\n");
            meta.clear_iptc();
            info!("> Cleared all IPTC data!\n");
        }

        let new_path = if keep_filename {
            image_path.to_owned()
        } else {
            let filename_stem = image_path.file_stem().unwrap_or(OsStr::new(""));
            let mut new_filename = filename_stem.to_os_string();
            new_filename.push("-scrubbed");

            utils::change_file_name(image_path, new_filename.to_str().unwrap())
        };

        if keep_filename {
            if let Ok(_) = std::fs::copy(image_path, &new_path) {
                info!("> Copied the file");
            } else {
                warn!("> Could not copy the image to the new path");
            }
        }

        if meta.save_to_file(new_path).is_ok() {
            info!("> Scrubbed image saved successfully");
            Ok(())
        } else {
            warn!("> Scrubbed image could not be saved");
            Err("Saving image failed".into())
        }
    } else {
        warn!("> Error: could not scrub image (maybe already scrubbed?)");
        Err("Scrubbing failed (maybe saving or copying failed?)".into())
    }
}

pub fn convert_whole_dir(
    base_path: &std::path::Path,
    keep_filename: bool,
    recursive: bool,
) -> Result<(), Box<dyn Error>> {
    if !recursive {
        // Only top level dir
        for entry in fs::read_dir(base_path)? {
            {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let _ = scrub_image_file(path.as_path(), keep_filename);
                }
            }
        }

        Ok(())
    } else {
        // Recursive scrubbing
        let mut total = 0;
        for entry in WalkDir::new(base_path)
            .follow_links(true)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            {
                total += 1;
                let image_path = entry.path();
                let _ = scrub_image_file(image_path, keep_filename);
            }
        }
        println!("Scrubbed {} images in total.", total);
        Ok(())
    }
}
