use clap::Parser;

mod scrubber;

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

    // Check if the file exists
    let image_path = &args.path;

    scrubber::scrub_image_file(image_path, false);
}
