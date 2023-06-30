use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    /// C function that acts as an interface between BDumper and the windows debug function
    /// UnDecorateSymbolName
    ///
    /// # Arguments
    ///
    /// * `s`: A C string that holds the MSVC symbol to be demangled
    ///
    /// returns: *const i8
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn demangle(s: *const c_char) -> *const c_char;
}

/// A wrapper for the C demangling function in order to isolate the unsafe code
///
/// # Arguments
///
/// * `symbol`: A reference to a string that contains the MSVC symbol to demangle
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn undecorate(symbol: &str) -> String {
    unsafe {
        let cstr = CString::new(symbol).unwrap();
        let result = CStr::from_ptr(demangle(cstr.as_ptr()));

        return match result.to_str() {
            Ok(res) => res.to_string(),
            Err(_) => "Failed to demangle".to_string(),
        };
    }
}

/// Formats already demangled symbols to make the output nicer and more readable
///
/// # Arguments
///
/// * `symbol`: Demangled function prototype
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn cleanup_symbol(symbol: &str) -> String {
    let res = undecorate(symbol);
    let demangled_name = res.replace("const", " const").replace("(", "( ");
    let mut declaration: Vec<&str> = demangled_name.split(" ").collect();

    for i in 0..declaration.len() {
        if &declaration[i] as &str == "const" && declaration[i - 1].starts_with("__") && i != 0 {
            let check_space = if &declaration[i - 1] as &str == " " {
                i - 1
            } else {
                i - 2
            };

            declaration.swap(i, check_space);
        }
    }

    declaration
        .join(" ")
        .replace("class", "")
        .replace("struct", "")
        .replace("  ", " ")
        .replace("   ", " ")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" &", "&")
        .replace(" *", "*")
        .replace("( ", "(")
}
