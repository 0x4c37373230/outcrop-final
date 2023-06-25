use std::thread;
use subprocess::Exec;

pub fn bds_thread() {
    thread::spawn(|| match Exec::shell("bedrock_server.exe").join() {
        Err(e) => {
            nwg::simple_message("Error", &format!("{}", e));
        }
        _ => {}
    });
}
