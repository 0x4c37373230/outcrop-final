use std::collections::HashMap;

use crate::files::path_exists;

pub fn loader_files(dll_path: &str) -> bool {
    let dll_folder = String::from("plugins");
    let bds_path = String::from("../bedrock_server.exe");

    if !path_exists(&dll_folder) || !path_exists(&bds_path) || !path_exists(&dll_path) {
        return false;
    }
    true
}

pub fn dll_map() -> Result<HashMap<i32, String>, i32> {
    let dll_folder = String::from("./plugins");

    if !path_exists(&dll_folder) {
        return Err(-1);
    }

    let dll_paths = std::fs::read_dir(&dll_folder).unwrap();
    let mut dll_number = 0;
    let mut dll_list: HashMap<i32, String> = HashMap::new();

    for path in dll_paths {
        let dll = path.unwrap().path().display().to_string();

        if dll.ends_with(".dll") {
            dll_number += 1;
            dll_list.insert(dll_number, dll);
        }
    }

    Ok(dll_list)
}
