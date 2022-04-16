# exiftool-rs

Simple image metadata scrubber.
Will remove EXIF, XMP and IPTC metadata.

## Packages

 - [Arch User Repository](https://aur.archlinux.org/packages/exiftool-rs-git)

## Usage

```
Jae Lo Presti
A small tool to scrub metadata from images.

USAGE:
    exiftool-rs [OPTIONS] <PATH>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SWITCHES:
    -i, --inplace    Do not change the image name

FILE:
    <PATH>    The image you want to apply the changes to
```

## Compiling

```
git clone https://github.com/jae1911/exiftool-rs
cd exiftool-rs
cargo build -r
```

The program will be then be available in `target/release/exiftool-rs`
