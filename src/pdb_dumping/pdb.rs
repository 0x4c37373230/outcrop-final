use crate::pdb_dumping::demangling;
use {
    pdb::{FallibleIterator, Rva},
    std::io::{BufRead, BufReader},
    std::{fs::File, io::Write, time::Instant},
};

pub struct BDSFunction {
    pub name: String,
    pub symbol: String,
    pub rva: Rva,
}

impl BDSFunction {
    /// This function acts as a constructor for BDSFunction instances
    ///
    /// # Arguments
    ///
    /// * `name`: Symbol (function or constant) name
    /// * `symbol`: Mangled function symbol
    /// * `rva`: Relative virtual address of the symbol
    ///
    /// returns: BDSFunction
    fn create_instance(name: String, symbol: String, rva: Rva) -> BDSFunction {
        return BDSFunction { name, symbol, rva };
    }
}

///
///
/// # Arguments
///
/// * `pdb_path`: Location of the pdb file to be dumped
/// * `file_type`: Dump result output format
/// * `dump_file`: Handle to the file to write the PDB contents to
/// * `should_demangle`: Determine if names should be demangled or not (the latter increases
/// performance)
///
/// returns: Result<(), Error>
pub fn pdb_dump(
    pdb_path: &str,
    file_type: &str,
    mut dump_file: File,
    should_demangle: bool,
) -> pdb::Result<()> {
    let start = Instant::now();
    let file_path = File::open(&pdb_path)?;
    let mut pdb = pdb::PDB::open(file_path)?;
    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;
    let mut symbols = symbol_table.iter();

    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(pdb::SymbolData::Public(data)) if data.function => {
                let rva = data.offset.to_rva(&address_map).unwrap_or_default();

                if file_type == ".txt" {
                    if should_demangle {
                        write!(
                            dump_file,
                            "{}\n{}\n{}\n\n",
                            data.name,
                            demangling::cleanup_symbol(&data.name.to_string()),
                            rva
                        )
                    } else {
                        write!(dump_file, "{}\n{}\n\n", data.name, rva)
                    }
                    .expect("ERROR: Could not write to file");
                } else if file_type == ".hpp" {
                    if should_demangle {
                        write!(
                            dump_file,
                            "//{}\n//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                            data.name,
                            demangling::cleanup_symbol(&data.name.to_string()),
                            md5::compute(data.name.to_string().to_string()),
                            rva
                        )
                        .expect("ERROR: Could not write to file");
                    } else {
                        write!(
                            dump_file,
                            "//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                            data.name,
                            md5::compute(data.name.to_string().to_string()),
                            rva
                        )
                        .expect("ERROR: Could not write to file");
                    }
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
    nwg::simple_message(
        "Completed",
        &format!("Completed dumping {} in {:?}", pdb_path, start.elapsed()),
    );

    Ok(())
}

/// Similar to the function above except that this returns the data corresponding to only one
/// function therefore making it more performant since only the function symbol is checked every
/// iteration, no data writing and nothing else is accessed
///
/// # Arguments
///
/// * `pdb_path`: Location of the PDB file to be dumped
/// * `function_name`: Function name to find formatted as [Class Name]::[Function]
///
/// returns: Result<BDSFunction, String>
pub fn find_function(pdb_path: &str, function_name: &str) -> Result<BDSFunction, String> {
    match File::open(&pdb_path) {
        Ok(file_path) => {
            let mut pdb = pdb::PDB::open(file_path).unwrap();
            let symbol_table = pdb.global_symbols().unwrap();
            let address_map = pdb.address_map().unwrap();
            let mut symbols = symbol_table.iter();

            while let Some(symbol) = symbols.next().unwrap() {
                match symbol.parse() {
                    Ok(pdb::SymbolData::Public(data)) if data.function => {
                        let symbol = data.name.to_string().to_string();
                        let rva = data.offset.to_rva(&address_map).unwrap_or_default();
                        let function_sym: Vec<&str> = function_name.split("::").collect();
                        let substr = format!("{}@{}", function_sym[1], function_sym[0]);

                        if symbol.contains(&substr) {
                            let found_function = BDSFunction::create_instance(
                                demangling::cleanup_symbol(&data.name.to_string()),
                                symbol,
                                rva,
                            );
                            return Ok(found_function);
                        }
                    }
                    _ => {}
                }
            }

            Err(String::from(
                "Function was either not found or does not exist",
            ))
        }
        Err(_) => Err(String::from(format!("File does not exist: {}", pdb_path))),
    }
}

/// Same as above but it finds multiple functions that should be written down in the filter file
/// generated by this program. Less performant
///
/// # Arguments
///
/// * `pdb_path`: Location of the PDB to be dumped
/// * `file_type`: Output format
/// * `dump_file`: Output file handle
///
/// returns: Result<(), String>
pub fn find_functions(pdb_path: &str, file_type: &str, mut dump_file: File) -> Result<(), String> {
    let file = File::open("./dumpFilter.txt").unwrap();
    let functions = BufReader::new(file);

    for line in functions.lines() {
        let line_ref: &str = &line.unwrap();

        if line_ref.starts_with("#") || line_ref.is_empty() {
            continue;
        }

        match find_function(pdb_path, line_ref) {
            Ok(bds_func) => {
                if file_type == ".txt" {
                    write!(
                        dump_file,
                        "{}\n{}\n{}\n\n",
                        &bds_func.symbol,
                        demangling::cleanup_symbol(&bds_func.symbol),
                        bds_func.rva
                    )
                    .expect("ERROR: Could not write to file");
                } else if file_type == ".hpp" {
                    write!(
                        dump_file,
                        "//{}\n//{}\nconstexpr unsigned int MD5_{:x} = {};\n\n",
                        &bds_func.symbol,
                        demangling::cleanup_symbol(&bds_func.symbol),
                        md5::compute(&bds_func.symbol),
                        bds_func.rva
                    )
                    .expect("ERROR: Could not write to file");
                }
            }
            Err(str) => {
                return Err(str);
            }
        }
    }

    nwg::simple_message(
        "Completed",
        &format!("Completed filtered dumping of {}", pdb_path),
    );
    Ok(())
}
