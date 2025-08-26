use std::env;
use std::path::Path;

include!("./build_support.rs");

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    
    generate_config_if_needed(dest_path);
}