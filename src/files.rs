/// Checks whether a given file or folder exists
///
/// # Arguments
///
/// * `path`: Path to the file or folder to be checked
///
/// returns: bool
pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
