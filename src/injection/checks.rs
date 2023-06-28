use std::collections::HashMap;

use crate::files::path_exists;

/// Checks whether the DLL the user wants to inject, the target (bedrock_server.exe) and the plugins
/// folder exist
///
/// # Arguments
///
/// * `dll_path`: Presumed location of the DLL the user wants to inject
///
/// returns: bool
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

    // if the plugins folder (which is presumed to contain all the BDS mods is found)
    let dll_paths = std::fs::read_dir(&dll_folder).unwrap();
    let mut dll_number = 0; // will act as a way to give the DLL list an order
    let mut dll_list: HashMap<i32, String> = HashMap::new();

    for path in dll_paths {
        let file = path.unwrap().path().display().to_string();

        // for every file in the plugins folder check the file path to see if the suffix is .dll
        // if the file is in fact a DLL, add it to the list of DLL (HashMap)
        if file.ends_with(".dll") {
            dll_number += 1;
            dll_list.insert(dll_number, file);
        }
    }

    Ok(dll_list)
}
