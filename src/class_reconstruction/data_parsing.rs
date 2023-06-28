pub struct ClassInfo {
    pub var_names: Vec<String>,
    pub offsets: Vec<i64>,
}

pub enum FunctionType {
    GETTERS,
    SETTERS,
}

/// parses the member variable name into a string to be used in the name for a getter or setter
/// by removing the 1st 2 characters of the variable if it's a pointer, or only the 1st
/// character if it's a normal variable. in all cases ';', the last character, is removed,
/// else you won't get valid C++ syntax (as if that was not obvious)
///
/// # Arguments
///
/// * `name`: member variable name
///
/// # Examples
///
/// ```
/// format!("get{}() {{\n\t;\n}}", name_for_method("*mLegacyBlock"));
/// // will return "getLegacyBlock() {\n\t;\n}"
/// ```
pub fn name_for_method(name: &str) -> &str {
    let name_len = name.len() - 1;

    return if name.starts_with("m") {
        &name[1..name_len]
    } else if name.starts_with("*m") {
        &name[2..name_len]
    } else {
        &name[0..name_len]
    };
}

/// Determines whether the reinterpret_cast should or shouldn't be dereferenced based on whether the
/// return type is or is not a pointer
///
/// # Arguments
///
/// * `buffer`: getter return type
///
/// # Examples
///
/// ```
/// cast_type("BlockPos *"); // will return "reinterpret_cast"
/// ```
pub fn cast_type(buffer: &str) -> &str {
    return if buffer.contains("*") {
        "reinterpret_cast"
    } else {
        "*reinterpret_cast"
    };
}

/// Gets the offset for a specific member variable from the class memory layout extracted from the
/// IDA 'Structures' section
///
/// # Arguments
///
/// * `line`: line from the input file inside the memory layout section
/// * `line_contents`: contents of the line, to easily be able to get the offset
/// * `member_names`: member variable name list
pub fn get_offset<'a>(
    line: &str,
    line_contents: &'a Vec<&str>,
    member_names: &Vec<String>,
) -> Result<&'a str, i64> {
    for name in member_names {
        if line.contains(name) {
            // if the line in the memory layout contains the name of a class member, then
            // this will return a string with the first element of said line, that being the
            // offset
            return Ok(line_contents[0]);
        }
    }

    Err(-1)
}
