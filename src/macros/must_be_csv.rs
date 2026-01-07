#[macro_export]
macro_rules! ensure_csv {
    ($path:expr, $ext:expr) => {
        let sad_emoji: char = '\u{1F643}';
        let path = std::path::Path::new($path);
        let is_valid = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map_or(false, |ext| ext == $ext);

        if !is_valid {
            eprintln!(
                "Error:{} The file {:?} is not a .{} file!",
                sad_emoji, path, $ext
            );
            std::process::exit(1);
        }
    };
}
