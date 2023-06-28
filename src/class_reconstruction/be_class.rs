use crate::class_reconstruction::{
    data_parsing::{cast_type, name_for_method, ClassInfo, FunctionType},
    input_file::InputFile,
};

/// Contains or will contain the C++ file format of the class being reconstructed
pub struct BEClass {
    pub beginning: String, // first line of the class, containing name and keywords
    pub members: String,   // contains declarations of class members
    getters: String,
    setters: String,
    pub data: ClassInfo, // contains two vectors of data, the first has member names,
} // the second has offsets

impl std::fmt::Display for BEClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.beginning, self.getters, self.setters)
    }
}

impl BEClass {
    /// Constructor for BEClass
    pub fn new() -> Self {
        Self {
            beginning: "".to_string(),
            members: "".to_string(),
            getters: "".to_string(),
            setters: "".to_string(),
            data: ClassInfo {
                var_names: Vec::new(),
                offsets: Vec::new(),
            },
        }
    }

    /// Generates the output class body using previously parsed data and stores it in BEClass
    /// members
    ///
    /// # Arguments
    ///
    /// * `file_structure`: Contains the position of the lines that delimit the file sections
    pub fn setup(&mut self, file_structure: &InputFile) -> Result<(), bool> {
        match file_structure.check_integrity() {
            Ok(_) => {
                let members = self.members.clone();
                // a vector of member declarations; and a vector of vectors of member declarations
                let member_variables: Vec<&str> = members.split("\n").collect();
                let mut new_methods: Vec<Vec<&str>> = Vec::new();

                for line in member_variables {
                    // for every member declaration, transform it into a vector splitting on
                    // whitespaces, then push the new vector to the new_methods variable
                    new_methods.push(line.trim().split(" ").collect())
                }

                // generate both getters and setters for all class members
                self.generate_methods(&new_methods, FunctionType::GETTERS);
                self.generate_methods(&new_methods, FunctionType::SETTERS);

                Ok(())
            }
            Err(_) => Err(false),
        }
    }

    /// Generates a list of getters and setters without the need for member variables using only
    /// reinterpret casts and the class `this` pointer
    ///
    /// # Arguments
    ///
    /// * `fields`: contains the member variable names and types
    /// * `fn_type`: determines whether to generate getters or setters
    fn generate_methods(&mut self, fields: &Vec<Vec<&str>>, fn_type: FunctionType) {
        // TODO: Proper commenting and documentation (explain this code past-Luke!)
        let mut index: usize = 0;

        for str_vec in fields {
            let mut buf_str: String = String::from("");
            let mut var_name: String = String::from("");

            for &str in str_vec {
                if !(str.starts_with("m") || str.starts_with("*m")) {
                    buf_str.push_str(str);
                    buf_str.push_str(" ");
                } else {
                    var_name.push_str(str);

                    if str.contains("*") {
                        buf_str.push_str("*");
                    }
                }
            }
            if &buf_str != " " && &var_name != " " {
                match fn_type {
                    FunctionType::GETTERS => {
                        self.getters.push_str(&format!("\tauto get{}() -> {}{{\n\t\treturn {}<{}>(reintepret_cast<VA>(this) + 0x{:X}); \n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), self.data.offsets[index]));
                    }
                    FunctionType::SETTERS => {
                        self.setters.push_str(&format!("\tauto set{}({} param_1) -> void {{\n\t\t{}<{}>(reintepret_cast<VA>(this) + 0x{:X}) = param_1;\n\t}}\n", name_for_method(&var_name), &buf_str, cast_type(&buf_str), &buf_str.trim(), self.data.offsets[index]))
                    }
                }
            }
            index += 1;
        }
    }
}
