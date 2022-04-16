use rexiv2::Metadata;
use std::ffi::OsStr;

mod utils;

pub fn scrub_image_file(image_path: &std::path::Path, keep_filename: bool) {
    println!("> Found a path {}, processing!\n", image_path.display());

    println!("\n> Attempting to clean...\n");

    let meta = Metadata::new_from_path(image_path).unwrap_or_else(|_error| {
                panic!("There was a problem when scrubbing the image, maybe it was already scrubbed?");
            });

    // EXIF
    if meta.supports_exif() {
        println!("> EXIF data found!\n");
        meta.clear_exif();
        println!("> Cleared all EXIF data!\n");
    } else {
        println!("> No EXIF data found (or not supported)\n.");
    }

    // XMP
    if meta.has_xmp() {
        println!("> XMP data found!\n");
        meta.clear_xmp();
        println!("> Cleared all XMP data!\n");
    } else {
        println!("> No XMP data found (or not supported)\n");
    }

    // IPTC
    if meta.has_iptc() {
        println!("> IPTC data found!\n");
        meta.clear_iptc();
        println!("> Cleared all IPTC data!\n");
    } else {
        println!("> No IPTC data found (or not supported)\n");
    }

    // Generate new path for image
    if !keep_filename {
        let filename_stem = image_path.file_stem().unwrap_or(OsStr::new(""));
        let mut new_filename = filename_stem.to_os_string();
        new_filename.push("-scrubbed");

        let new_path = utils::change_file_name(image_path, new_filename.to_str().unwrap());
        println!("> Saving modified image to {:?}", new_path);

        _ = std::fs::copy(image_path.as_os_str(), new_path.as_os_str());
        _ = meta.save_to_file(new_path);
    } else {
        println!("> Saving modified image to {:?}", image_path);
        _ = std::fs::copy(image_path.as_os_str(), image_path.as_os_str());
        _ = meta.save_to_file(image_path);
    }
    
}
