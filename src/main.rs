#[path = "running/execute.rs"] mod execute;

use std::fs;

fn main() {
    let file_path = "code.itm";

    let compiled_code = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    execute::run_code(compiled_code.to_string());
}