use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

const FILE_EXTENSIONS_SUPPORTED: &[&str] = &["jpeg", "jpg", "tiff", "wav", "png", "webp"];
pub fn check_can_be_scrubbed(path: impl AsRef<Path>) -> bool {
    if let Some(extension) = path.as_ref().extension().and_then(OsStr::to_str) {
        FILE_EXTENSIONS_SUPPORTED.contains(&extension.to_lowercase().as_str())
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::scrubber::utils::change_file_name;
    use crate::scrubber::utils::check_can_be_scrubbed;

    use std::ffi::OsStr;
    use std::path::Path;

    #[test]
    fn change_file_name_works() {
        let image_path = Path::new("test.jpeg");
        let filename_stem = image_path.file_stem().unwrap_or(OsStr::new(""));
        let mut new_filename = filename_stem.to_os_string();
        new_filename.push("-scrubbed");

        assert_eq!(
            Path::new("test-scrubbed.jpeg"),
            change_file_name(image_path, new_filename.to_str().unwrap())
        );
    }

    #[test]
    fn image_is_compatible() {
        let image_path = Path::new("test.jpeg");
        let result = check_can_be_scrubbed(image_path);
        assert_eq!(true, result);
    }

    #[test]
    fn image_is_not_compatible() {
        let image_path = Path::new("test.gif");
        let result = check_can_be_scrubbed(image_path);
        assert_eq!(false, result);
    }

    #[test]
    fn image_is_compatible_uppercase() {
        let image_path = Path::new("test.JPG");
        let result = check_can_be_scrubbed(image_path);
        assert_eq!(true, result);
    }

    #[test]
    fn image_is_not_compatible_uppercase() {
        let image_path = Path::new("test.GIF");
        let result = check_can_be_scrubbed(image_path);
        assert_eq!(false, result);
    }
}
