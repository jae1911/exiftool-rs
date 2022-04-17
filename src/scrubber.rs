use crate::scrubber::utils::check_can_be_scrubbed;
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
    if !check_can_be_scrubbed(image_path.to_owned()) {
        info!("> Image not compatible with scrubbing")
    }

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

        let new_path = if !keep_filename {
            let filename_stem = image_path.file_stem().unwrap_or(OsStr::new(""));
            let mut new_filename = filename_stem.to_os_string();
            new_filename.push("-scrubbed");

            utils::change_file_name(image_path, new_filename.to_str().unwrap())
        } else {
            image_path.to_owned()
        };

        if !keep_filename {
            let copy_result = std::fs::copy(image_path, &new_path);
            match copy_result {
                Ok(_) => info!("> Copied the file"),
                Err(e) => warn!("> An error happened: {}", e),
            }
        }

        let saving_result = meta.save_to_file(new_path);
        match saving_result {
            Ok(_) => {
                info!("> Scrubbed image saved successfully");
                Ok(())
            },
            Err(e) => {
                warn!("> Scrubbed image could not be saved: {}", e);
                Err("Saving image failed".into())
            },
        }
    } else {
        info!("> Error: could not scrub image (maybe already scrubbed?)");
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
                let image_path = entry.path();
                if image_path.is_file() {
                    let scrub_result = scrub_image_file(image_path, keep_filename);

                    match scrub_result {
                        Ok(_) => total += 1,
                        Err(e) => info!("> An error happened: {}", e),
                    }
                }
            }
        }
        println!("Scrubbed {} images in total.", total);
        Ok(())
    }
}
