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

#[cfg(test)]
mod tests {
    use crate::scrubber::utils::change_file_name;
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
}
