use clap::Parser;
use walkdir::WalkDir;

mod scrubber;

#[derive(Parser, Debug)]
#[clap(author = "Jae Lo Presti", version, about = "A small tool to scrub metadata from images.")]
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
    /// Verbose output
    #[clap(
        help_heading = Some("SWITCHES"),
        short,
        long
    )]
    verbose: bool,
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
    let verbose = args.verbose;

    if image_path.exists() && image_path.is_file() {
        println!("> Scrubbing a single file\n");
        // Scrub single image
        scrubber::scrub_image_file(image_path, keep_filename, verbose);
    } else if image_path.exists() && image_path.is_dir() {
        // Scrub whole dir
        if scrub_directory {
            let mut total = 0;
            println!("> Alright, attempting to scrub the directory!\n");
            for entry in WalkDir::new(image_path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) {
                    total += 1;
                    {
                        let image_path = entry.path();
                        scrubber::scrub_image_file(image_path, keep_filename, verbose);
                    }
                }
            println!("Scrubbed a total of {} files", total);
        } else {
            println!("> You are attempting to scrub a whole directory; to confirm, please run the command with the -d (or --directory) switch");
        }
    } else {
        println!("> Warning, no file found");
    }

}
