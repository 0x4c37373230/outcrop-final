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

pub fn inject_mod(dll_path: &str) {
    thread::spawn(|| match Exec::shell("bedrock_server.exe").join() {
        Err(err) => {
            msg_builder(false, &err.to_string());
            return
        }
        _ => {}
    });

    thread::sleep(time::Duration::from_millis(5000));

    // TODO: Change the temporary use of injrs to my own DLL injector code
    let bds_process = Process::find_first_by_name("bedrock_server.exe").unwrap();

    match bds_process.inject(dll_path) {
        Ok(_) => msg_builder(true, "Successfully injected DLL"),
        Err(err) => msg_builder(false, &err.to_string())
    }
}
