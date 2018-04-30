use std::fs;
//use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::error::Error;

fn main() {
    //let files_location = String::from("../input_data/stocks/Test/");
    //let v = get_names_of_symbol_files(&files_location);
    //for i in v {
    //    let symbol = extract_symbol(i);
    //    println!("{}", symbol);
    //}
    dump_symbols_to_file("../output_data/stocks/", "symbols.csv", "NEW");
}

fn create_file(file_location:String, file_name:String, mode:String) {
    let path = Path::new("../output_data/stocks/symbols.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

}

/// Returns a ticker symbol as a String
///
/// # Arguments
///
/// * `file_name` - name of the file to extact the symbol from.
///
/// # Returns
///
/// * `symbol` - symbol name as a string
///
/// # Remarks
///
/// This is a convenience function that extract the stock sumbol name
pub fn extract_symbol(file_name: std::ffi::OsString) -> String{
    let s:String = file_name.into_string().unwrap();
    let dot_index = s.find('.').unwrap();
    let symbol: String = s.chars().skip(0).take(dot_index).collect();
    //println!("{}", symbol);
    return symbol
}

/// Returns vector of file names where each element is the name of the file
///
/// # Arguments
///
/// * `location` - folder location of the input files.
///
/// # Returns
///
/// * `v` - vector of file names.
///
/// # Remarks
///
/// This is a convenience function reads the contents of `location` and puts the file names into a vector
pub fn get_names_of_symbol_files(location: &str) -> Vec<std::ffi::OsString>{
    let mut v = Vec::new();
    if let Ok(entries) = fs::read_dir(location) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                v.push(entry.file_name());
            }
        }
    }
    return v
}
