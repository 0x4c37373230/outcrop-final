#![windows_subsystem = "windows"]

extern crate alloc;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

mod class_reconstruction;
mod files;
mod injection;
mod pdb_dumping;

use crate::pdb_dumping::setup;
use class_reconstruction::{
    be_class::BEClass,
    input_file::{loop_through_file, InputFile},
};
use files::path_exists;
use injection::{checks::loader_files, mod_loading::inject_mod};
use nwg::{CheckBoxState, NativeUi};
use pdb_dumping::pdb;
use std::{fs::File, io::Write};

/// Contains the UI layout. The component prefixes indicate which tool something
/// belongs to. the 'd' prefix indicates something corresponds to the PDB dumper. The 'i'
/// prefix indicates something corresponds to the DLL injector, and the 'cr' prefix indicates
/// something corresponds to the IDA class reconstructor
#[derive(Default, nwd::NwgUi)]
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
    #[nwg_events( OnButtonClick: [Outcrop::find] )]
    d_find_function: nwg::Button,

    #[nwg_control(text: "Dump Data", size: (235, 30), position: (10, 275))]
    #[nwg_events( OnButtonClick: [Outcrop::dump] )]
    d_dump_data: nwg::Button,

    #[nwg_control(text: "Filtered Dump", size: (235, 30), position: (250, 275))]
    #[nwg_events( OnButtonClick: [Outcrop::filtered_dump] )]
    d_filtered_dump: nwg::Button,

    #[nwg_control(text: "Input DLL path to inject", size: (235, 25), position: (250, 80))]
    i_label1: nwg::Label,
    #[nwg_control(text: "", size: (235, 25), position: (250, 100))]
    i_dll_path: nwg::TextInput,

    #[nwg_control(text: "See DLL list", size: (235, 25), position: (250, 130))]
    i_label2: nwg::Label,
    #[nwg_control(text: "Available mods", size: (235, 25), position: (250, 150))]
    #[nwg_events( OnButtonClick: [Outcrop::list_mods] )]
    i_mod_list: nwg::Button,

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
    fn dump(&self) {
        let pdb_path: &str = &self.d_pdb_path.text();
        let file_type: &str = &self.d_dump_file_type.text();
        let demangle = if &self.d_should_demangle.check_state() == &CheckBoxState::Checked {
            true
        } else {
            false
        };

        match setup::dump_init(pdb_path, file_type) {
            Ok(dump_file) => pdb::pdb_dump(pdb_path, file_type, dump_file, demangle)
                .expect("ERROR: Failed to dump pdb contents"),
            Err(msg) => {
                msg_builder(false, &msg);
                return;
            }
        }
    }

    fn find(&self) {
        match pdb::find_function(&self.d_pdb_path.text(), &self.d_func_name.text()) {
            Ok(bds_func) => nwg::simple_message(
                "Found a match",
                &format!(
                    "Function name: {}\nSymbol: {}\nRVA: {}",
                    bds_func.name, bds_func.symbol, bds_func.rva
                ),
            ),
            Err(msg) => nwg::error_message("ERROR", &msg),
        };
    }

    fn filtered_dump(&self) {
        let pdb_path: &str = &self.d_pdb_path.text();
        let file_type: &str = &self.d_dump_file_type.text();

        match setup::dump_init(pdb_path, file_type) {
            Ok(dump_file) => match pdb::find_functions(pdb_path, file_type, dump_file) {
                Err(msg) => msg_builder(false, &msg),
                _ => {}
            },
            Err(msg) => msg_builder(false, &msg),
        }
    }

    fn inject(&self) {
        // get the DLL path from user input
        let dll_path = String::from(&self.i_dll_path.text());

        // if the DLL, plugins folder, and BDS executable are found, then inject the DLL
        if loader_files(&dll_path) {
            inject_mod(&dll_path);
        } else {
            msg_builder(false, "Could not find a file. Check whether all the files required for injection do in fact exist");
        }
    }

    // NOT USABLE (YET) UNLESS OUTCROP IS INSIDE THE BDS FOLDER
    fn list_mods(&self) {
        // get DLL ordered list from plugins folder (if it exists)
        match injection::checks::dll_map() {
            Ok(dll_list) => {
                let mut dlls: String = String::from("");
                // loop through the ordered DLL list and add a line to the string that will be
                // displayed, containing the number assigned to the DLL and it's name
                for (key, dll) in &dll_list {
                    dlls.push_str(&format!("{}: {}\n", key, dll));
                }

                msg_builder(true, &format!("{}", dlls));
            }
            Err(_) => msg_builder(false, "Could not find 'plugins' folder"),
        };
    }

    fn reconstruct_class(&self) {
        // get location of 'class.txt' from user input
        let class_template_path = self.cr_class_file_path.text();

        match File::open(&class_template_path) {
            // if said file can be opened...
            Ok(file) => {
                let mut file_structure = InputFile::new();
                let mut out_class = BEClass::new();

                // ...loop through it, process the sections, get preliminary class information, and
                // store everything in the variables declared above
                loop_through_file(&file, &mut file_structure, &mut out_class);

                // attempt to generate C++ class body and members
                match out_class.setup(&file_structure) {
                    // if this succeeds, attempt to create a C++ header file to contain the output
                    // code and attempt to write the class code to it
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
            Err(_) => {
                if path_exists(&class_template_path) {
                    msg_builder(false, "Could not open class file")
                } else {
                    msg_builder(false, "Class file does not exist")
                }
            }
        };
    }

    fn exit_program(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    setup::filter_manager();

    let _app = Outcrop::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

/// Displays a windows message box with either the output or an error
///
/// # Arguments
///
/// * `code`: a boolean. true determines a success while false determines an error
/// * `content`: this text to be be displayed on the message box
fn msg_builder(code: bool, content: &str) {
    if code {
        nwg::simple_message("SUCCESS!", content);
    } else {
        nwg::error_message("ERROR", content);
    }
}
