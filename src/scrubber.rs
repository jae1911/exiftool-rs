use rexiv2::Metadata;
use std::ffi::OsStr;
use std::fs;
use std::io;
use walkdir::WalkDir;

mod utils;

pub fn scrub_image_file(image_path: &std::path::Path, keep_filename: bool, verbose: bool) {
    if verbose {
        println!("> Found a path {}, processing!\n", image_path.display());

        println!("\n> Attempting to clean...\n");
    }

    if let Ok(meta) = Metadata::new_from_path(image_path) {
        // EXIF
        if meta.supports_exif() {
            if verbose {
                println!("> EXIF data found!\n");
            }
            meta.clear_exif();
            if verbose {
                println!("> Cleared all EXIF data!\n");
            }
        } else if verbose {
            println!("> No EXIF data found (or not supported)\n");
        }

        // XMP
        if meta.has_xmp() {
            if verbose {
                println!("> XMP data found!\n");
            }
            meta.clear_xmp();
            if verbose {
                println!("> Cleared all XMP data!\n");
            }
        } else if verbose {
            println!("> No XMP data found (or not supported)\n");
        }

        // IPTC
        if meta.has_iptc() {
            if verbose {
                println!("> IPTC data found!\n");
            }
            meta.clear_iptc();
            if verbose {
                println!("> Cleared all IPTC data!\n");
            }
        } else if verbose {
            println!("> No IPTC data found (or not supported)\n");
        }

        // Generate new path for image
        if !keep_filename {
            let filename_stem = image_path.file_stem().unwrap_or(OsStr::new(""));
            let mut new_filename = filename_stem.to_os_string();
            new_filename.push("-scrubbed");

            let new_path = utils::change_file_name(image_path, new_filename.to_str().unwrap());
            if verbose {
                println!("> Saving modified image to {:?}", new_path);
            }

            _ = std::fs::copy(image_path.as_os_str(), new_path.as_os_str());
            _ = meta.save_to_file(new_path);
        } else {
            if verbose {
                println!("> Saving modified image to {:?}", image_path);
            }
            _ = std::fs::copy(image_path.as_os_str(), image_path.as_os_str());
            _ = meta.save_to_file(image_path);
        }
    } else if verbose {
        println!("> Error: could not scrub image (maybe already scrubbed?)");
    }
}

pub fn convert_whole_dir(
    base_path: &std::path::Path,
    keep_filename: bool,
    verbose: bool,
    recursive: bool,
) -> io::Result<()> {
    if !recursive {
        // Only top level dir
        for entry in fs::read_dir(base_path)? {
            {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    scrub_image_file(path.as_path(), keep_filename, verbose)
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
                scrub_image_file(image_path, keep_filename, verbose);
            }
        }
        println!("Scrubbed {} images in total.", total);
        Ok(())
    }
}
