#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod class_reconstruction;
mod injection;

use crate::class_reconstruction::be_class::BEClass;
use crate::class_reconstruction::input_file::{loop_through_file, InputFile};
use crate::injection::mod_loading::inject_mod;
use std::{fs::File, io::Write};
use {nwd::NwgUi, nwg::NativeUi};

#[derive(Default, NwgUi)]
/// This struct contains the UI layout. The component prefixes indicate which tool something
/// belongs to. the 'd' prefix indicates something corresponds to the PDB dumper. The 'i'
/// prefix indicates something corresponds to the DLL injector, and the 'cr' prefix indicates
/// something corresponds to the IDA class reconstructor
pub struct Outcrop {
    #[nwg_control(size: (500, 600), position: (300, 300), title: "OUTCROP", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Outcrop::exit_program] )]
    window: nwg::Window,

    #[nwg_control(text: "Outcrop: A BDS modding framework by Luke7720 to aid with reverse \
    engineering. Composed of three tools: a PDB dumper, a class generator, and a DLL injector. \
    Instructions below\n-----------------------------------------------------------------------\
    -----------------------------------------------------------\
    ", size: (475, 73), position: (10, 10))]
    info: nwg::Label,

    #[nwg_control(text: "Input PDB file path", size: (235, 25), position: (10, 80))]
    d_label1: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (10, 100))]
    d_pdb_path: nwg::TextInput,

    #[nwg_control(text: "Input file type (.txt/.hpp)", size: (235, 25), position: (10, 130))]
    d_label2: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (10, 150))]
    d_dump_file_type: nwg::TextInput,

    #[nwg_control(text: "Input desired function name", size: (235, 25), position: (10, 185))]
    d_label3: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (10, 210))]
    d_func_name: nwg::TextInput,

    #[nwg_control(text: "Include demangled prototypes", size: (235, 25), position: (10, 243))]
    d_should_demangle: nwg::CheckBox,

    #[nwg_control(text: "Find Function", size: (235, 30), position: (250, 240))]
    //    #[nwg_events( OnButtonClick: [] )]
    d_find_function: nwg::Button,

    #[nwg_control(text: "Dump Data", size: (235, 30), position: (10, 275))]
    //    #[nwg_events( OnButtonClick: [] )]
    d_dump_data: nwg::Button,

    #[nwg_control(text: "Filtered Dump", size: (235, 30), position: (250, 275))]
    //    #[nwg_events( OnButtonClick: [] )]
    d_filtered_dump: nwg::Button,

    #[nwg_control(text: "Input DLL absolute path to inject", size: (235, 25), position: (250, 80))]
    i_label1: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (250, 100))]
    i_dll_path: nwg::TextInput,

    #[nwg_control(text: "Inject", size: (235, 30), position: (250, 180))]
    #[nwg_events( OnButtonClick: [Outcrop::inject] )]
    i_inject: nwg::Button,

    #[nwg_control(text: "------------------------------------------------------------------\
    ", size: (235, 25), position: (250, 215))]
    line_separator1: nwg::Label,

    #[nwg_control(text: "-------------------------------------------------------------------\
    ---------------------------------------------------------------\
    ", size: (475, 25), position: (10, 310))]
    line_separator2: nwg::Label,

    #[nwg_control(text: "Input class txt file path", size: (235, 25), position: (10, 330))]
    cr_make_label1: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (10, 350))]
    cr_class_file_path: nwg::TextInput,

    #[nwg_control(text: "Reconstruct Class", size: (235, 50), position: (250, 330))]
    #[nwg_events( OnButtonClick: [Outcrop::reconstruct_class] )]
    cr_reconstruct_class: nwg::Button,

    #[nwg_control(text: "----------------------------------------------------------------------------------------------------------------------------------", size: (475, 25), position: (10, 380))]
    line_separator3: nwg::Label,
}

impl Outcrop {
    fn inject(&self) {
        let dll_abs_path = String::from(&self.i_dll_path.text());

        match inject_mod(&dll_abs_path) {
            Ok(()) => msg_builder(true, "Successfully injected DLL"),
            Err(err) => msg_builder(false, &err.to_string()),
        };
    }

    fn reconstruct_class(&self) {
        match File::open(&self.cr_class_file_path.text()) {
            Ok(file) => {
                let mut file_structure = InputFile::new();
                let mut out_class = BEClass::new();

                loop_through_file(&file, &mut file_structure, &mut out_class);

                match out_class.setup(&file_structure) {
                    Ok(_) => match File::create("./class.hpp") {
                        Ok(mut out_file) => {
                            match out_file.write_all(format!("{}}};", out_class).as_ref()) {
                                Ok(_) => {
                                    msg_builder(true, "Successfully reconstructed class from file")
                                }
                                Err(_) => msg_builder(false, "Could not write to file"),
                            }
                        }
                        Err(_) => msg_builder(false, "Could not create file"),
                    },
                    Err(_) => msg_builder(false, "Incorrect, incomplete, or invalid file format"),
                }
            }
            Err(_) => msg_builder(false, "Could not find class text file"),
        };
    }

    fn exit_program(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = Outcrop::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

fn msg_builder(code: bool, content: &str) {
    match code {
        true => nwg::simple_message("SUCCESS!", content),
        false => nwg::simple_message("ERROR", content),
    };
}
