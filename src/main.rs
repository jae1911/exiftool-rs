use std::ffi::OsStr;
use std::path::PathBuf;
use clap::Parser;

use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author = "Jae Lo Presti", version, about = "A small tool to scrub metadata from images.")]
struct Cli {
    /// The image you want to apply the changes to
    #[clap(
        help_heading = Some("FILE"),
        parse(from_os_str)
    )]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let result = std::fs::File::open(&args.path);
    match result {
        Ok(_content) => {
            println!("> Found a file, processing!\n");

            println!("\n> Attempting to clean...\n");

            let meta = rexiv2::Metadata::new_from_path(&args.path).unwrap();

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
            let filename_stem = &args.path.file_stem().unwrap_or(OsStr::new(""));
            let mut new_filename = filename_stem.to_os_string();
            new_filename.push("-scrubbed");

            let new_path = change_file_name(&args.path, new_filename.to_str().unwrap());

            println!("> Saving modified image to {:?}", new_path);

            _ = std::fs::copy(&args.path.as_os_str(), new_path.as_os_str());
            _ = meta.save_to_file(new_path);
        }
        Err(error) => { println!("An error occurred: {}", error); }
    }
}

fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}
