use crate::injection::bds;
use injrs::inject_windows::InjectorExt;
use injrs::process_windows::Process;
use std::{io, thread, time};

// This injection function does not seem to be working
/*
extern "C" {
    fn injectDll(process_id: i32, dll_path: *const c_char) -> bool;
    fn getProcId() -> u32;
}
*/

pub fn inject_mod(dll_abs_path: &str) -> Result<(), io::Error> {
    bds::bds_thread();
    thread::sleep(time::Duration::from_millis(5000));

    // TODO: Change the temporary use of injrs to my own DLL injector code
    let bds_process = Process::find_first_by_name("bedrock_server.exe").unwrap();

    return bds_process.inject(dll_abs_path);
}
