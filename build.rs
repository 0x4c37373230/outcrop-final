fn main() {
    cc::Build::new()
        //        .cpp(true)
        //        .file("src/injection/injectDll.cpp")
        //        .file("src/injection/getProcId.cpp")
        .file("src/pdb_dumping/demangling.c")
        .compile("helperExtraCode.a");
}
