use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::error::Error;

fn main() {
    let files_location = String::from("../input_data/stocks/Test/");
    let v = get_names_of_symbol_files(&files_location);
    let v2 = v.clone();
    //for i in v {
    //    let symbol = extract_symbol(i);
    //    println!("{}", symbol);
    //}

    let output_base_location = String::from("../output_data/stocks/");
    let symbols_file_name = String::from("symbols.csv");

    let file_create_mode = String::from("NEW");
    dump_symbols_to_file(&output_base_location, &symbols_file_name, &file_create_mode, v);

    let file_create_mode = String::from("APPEND");
    dump_symbols_to_file(&output_base_location, &symbols_file_name, &file_create_mode, v2);
}



pub fn dump_symbols_to_file(file_location: &str, file_name: &str, create_mode: &str, symbols_vect:  Vec<std::ffi::OsString>) {
    let file = concat_strings(&file_location,&file_name);
    if create_mode == "NEW" {
        create_file(file);
    }
    else if create_mode == "APPEND" {
        append_file(file, symbols_vect);
    }
}

pub fn append_file(file_loc_name:String, symbols_vect:  Vec<std::ffi::OsString>) {
    //TODO
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_loc_name)
        .unwrap();

    for i in symbols_vect {
        let symbol = extract_symbol(i);
        if let Err(e) = writeln!(file, symbol) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    //for i in v {
    //    let symbol = extract_symbol(i);
    //    println!("{}", symbol);
    //}
}

pub fn create_file(file_loc_name:String) {
    let path = Path::new(&file_loc_name);
    let display = path.display();

    let _file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(_file) => _file,
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

pub fn concat_strings(a: &str, b: &str) -> String{
    let c = [a, b].join("");
    return c
}
