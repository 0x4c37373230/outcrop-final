pub fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
