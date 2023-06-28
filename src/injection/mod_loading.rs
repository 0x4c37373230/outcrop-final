use crate::msg_builder;
use injrs::inject_windows::InjectorExt;
use injrs::process_windows::Process;
use std::{thread, time};
use subprocess::Exec;

// This injection function does not seem to be working
/*
extern "C" {
    fn injectDll(process_id: i32, dll_path: *const c_char) -> bool;
    fn getProcId() -> u32;
}
*/

/// Forces a process to load a DLL that will contain code to modify the BDS executable at runtime
///
/// # Arguments
///
/// * `dll_path`: Location of the DLL
pub fn inject_mod(dll_path: &str) {
    // start bedrock_server.exe as a detached process on a separate shell and wait for it to fire up
    thread::spawn(|| match Exec::shell("bedrock_server.exe").join() {
        Err(err) => {
            msg_builder(false, &err.to_string());
            return;
        }
        _ => {}
    });
    thread::sleep(time::Duration::from_millis(5000));

    // TODO: Change the temporary use of injrs to my own DLL injector code
    // try to find the BDS process
    match Process::find_first_by_name("bedrock_server.exe"){
        Some(bds_process) => {
            // if found, attempt to inject the desired DLL
            match bds_process.inject(dll_path) {
                Ok(_) => msg_builder(true, "Successfully injected DLL"),
                Err(err) => msg_builder(false, &err.to_string()),
            }
        }
        None => msg_builder(false, &err.to_string()),
    };
}
