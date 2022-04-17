#![warn(clippy::pedantic)]

use env_logger::Env;
use clap::Parser;
use log::{info, warn, error};

mod scrubber;

#[derive(Parser, Debug)]
#[clap(
    author = "Jae Lo Presti",
    version,
    about = "A small tool to scrub metadata from images."
)]
struct Cli {
    /// Do not change the image name
    #[clap(
        help_heading = Some("SWITCHES"),
        short,
        long
    )]
    inplace: bool,
    /// Scrub a whole directory (activator)
    #[clap(
        help_heading = Some("SWITCHES"),
        short,
        long
    )]
    directory: bool,
    /// Recursive scrubbing
    #[clap(
        help_heading = Some("SWITCHES"),
        short,
        long
    )]
    recursive: bool,
    /// The image you want to apply the changes to
    #[clap(
        help_heading = Some("FILE"),
        parse(from_os_str)
    )]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Check if the file exists
    let image_path = &args.path;
    let keep_filename = args.inplace;
    let scrub_directory = args.directory;
    let recursive = args.recursive;

    env_logger::init_from_env(Env::default().default_filter_or("warn"));

    if image_path.exists() && image_path.is_file() {
        info!("> Scrubbing a single file\n");
        // Scrub single image
        scrubber::scrub_image_file(image_path, keep_filename);
    } else if image_path.exists() && image_path.is_dir() {
        // Scrub whole dir
        if scrub_directory {
            info!("> Alright, attempting to scrub the directory!\n");

            let scrub_result =
                scrubber::convert_whole_dir(image_path, keep_filename, recursive);

            match scrub_result {
                Ok(_) => info!("> Scrubbing went without any errors"),
                Err(e) => warn!("> An error happened while scrubbing: {}", e),
            }
        } else {
            warn!("> You are attempting to scrub a whole directory; to confirm, please run the command with the -d (or --directory) switch\n> You can also use -r (--recursive) so it scours all the subfolders as well (in combination with the directory switch)");
        }
    } else {
        error!("> Warning, no file found");
    }
}
