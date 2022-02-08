use indexmap::IndexMap;

use llvmlite;

pub fn show_example_llvm() {
    let prog = llvmlite::Program {
        types: IndexMap::new(),
        globals: IndexMap::new(),
        functions: IndexMap::new(),
        externals: IndexMap::new(),
    };
    println!("{:?}", prog)
}
