use std::path::PathBuf;

pub fn tilde(path: PathBuf) -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let path_str = path.display().to_string();

    if path_str.starts_with(&home) {
        path_str.replacen(&home, "~", 1)
    } else {
        path_str
    }
}
