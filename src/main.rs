use clap::Parser;
use exif::{Reader};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// The operation you wish the cli to do (remove, randomize)
    operation: String,
    /// The image you want to apply the changes to
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let result = std::fs::File::open(&args.path);
    match result {
        Ok(content) => { 
            println!("> Found a file, processing!\n");
            let mut bufreader = std::io::BufReader::new(&content);
            let exif_reader = Reader::new();
            let exif = exif_reader.read_from_container(&mut bufreader).unwrap();
            println!("> Found the following EXIF data:");
            for f in exif.fields() {
                println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
            }
        }
        Err(error) => { println!("Error: {}", error); }
    }
}
