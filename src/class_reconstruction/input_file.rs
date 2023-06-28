use crate::class_reconstruction::{be_class::BEClass, data_parsing::get_offset};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Holds the basic structure of the input file (class.txt). class.txt contains 3 sections with
/// their respective delimiters. The declaration section holds the class declaration  and its
/// members. The memory section contains the memory layout of the class, and the end marks, well,
/// the end of the file
pub struct InputFile {
    pub dec_n: usize, // declaration delimiter line number
    pub mem_n: usize, // memory layout delimiter line number
    pub end_n: usize, // end of file line number
}

impl InputFile {
    // as a placeholder, delimiter positions are set to usize::MAX in order to later
    // perform checks regarding to whether the line was found or not in case there are
    // issues with how the input file was structured
    pub fn new() -> Self {
        Self {
            dec_n: usize::MAX,
            mem_n: usize::MAX,
            end_n: usize::MAX,
        }
    }

    // this function checks whether the input file has all delimiters in place. if the
    // delimiters contain usize::MAX, this means the value was not changed since that is
    // what is assigned in the constructor, therefore it also means a delimiter was not
    // found. if a delimiter is not found, the program cannot proceed, so it returns an
    // error and aborts
    pub fn check_integrity(&self) -> Result<bool, bool> {
        if self.dec_n == usize::MAX || self.mem_n == usize::MAX || self.end_n == usize::MAX {
            return Err(false);
        }

        Ok(true)
    }
}

/// Generates member name and offset lists, delimits the input file sections
///
/// # Arguments
///
/// * `file`: class.txt handle
/// * `file_structure`: reference to a class which will hold the position of the lines that delimit sections
/// * `temp_class`: temporary class which will hold temporary and new data being formatted from the txt file
pub fn loop_through_file(file: &File, file_structure: &mut InputFile, temp_class: &mut BEClass) {
    let reader = BufReader::new(file);
    let mut reset: bool = false;

    for (mut index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // for every line in the input file, if the file contains a line with a delimiter this then
        // stores the line number (index) into the corresponding delimiter
        if line.contains("DECLA") {
            file_structure.dec_n = index;
        } else if line.contains("INMEM") {
            file_structure.mem_n = index;
        } else if line.contains("ENDSF") {
            file_structure.end_n = index;
        }

        // this detects the first line of the class declaration and cleans it up, removing
        // alignment since it's not necessary, removing extras spaces, and actually using the
        // 'class' keyword as opposed to the IDA C-pseudocode alternative
        if line.contains("struct __cppobj") {
            let temp_string = String::from(&line);

            temp_class.beginning = temp_string
                // 'const' is not allowed in class declarations
                .replace("const struct __cppobj ", "class ")
                .replace("__declspec(align(8)) ", "")
                .replace("__declspec(align(4)) ", "");
            temp_class.beginning.push_str("\n{\npublic:\n");
        }

        // once all delimiters are actually found, the file will be read AGAIN to actually get the
        // class contents. the reset variable is included because this needs to happen only once,
        // else it'll recur eternally much like the universe according to Nietzsche
        if file_structure.dec_n != usize::MAX
            && file_structure.mem_n != usize::MAX
            && reset == false
        {
            index = 0;
            reset = true;
            continue;
        }

        let line_contents: Vec<&str> = line.split_whitespace().collect();

        if index > file_structure.dec_n
            && index < file_structure.mem_n
            && !(line.contains("{")
                || line.contains("};")
                || line.contains("/*VFT*/")
                || line.contains("struct __cppobj"))
        {
            // if the line is a part of the class declaration and it's not the start, end,  or a
            // comment indicating a vtable, then the entire line will be pushed into the 'members'
            // section of a BEClass
            temp_class.members.push_str(&format!("{}\n", line));
            // isolates a variable name and stores it, removing '*' denoting a pointer plus the ';'
            temp_class.data.var_names.push(
                String::from(line_contents[line_contents.len() - 1])
                    .replace("*", "")
                    .replace(";", ""),
            )
        } else if index > file_structure.mem_n {
            let actual_line = &line as &str;
            // else, if the line is a part of the class memory layout, get the offset string...
            match get_offset(actual_line, &line_contents, &temp_class.data.var_names) {
                Ok(offset) => temp_class
                    .data
                    .offsets
                    // ...convert it into base 16 i64, and push it to the offsets vector
                    .push(i64::from_str_radix(offset, 16).unwrap()),
                Err(_) => continue,
            }
        }
    }
}
