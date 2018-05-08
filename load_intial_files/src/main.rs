use std::fs;
use std::io::Write;

fn main() {
    let mut f = fs::File::create("output.txt").expect("Unable to create file");
    let data = get_names_of_symbol_files();
    for i in  data{
        //println!("{:?}", extract_symbol(i.symbol));
        let mut symbol = extract_symbol(i.symbol);
        f.write(symbol.as_bytes()).expect("Unable to write data");
        f.write(b"\n").expect("Unable to write data");
    }
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
pub fn extract_symbol(file_name: String) -> String{
    let last_slash_index = file_name.rfind('/').unwrap();
    let last_dot_index = file_name.rfind('.').unwrap();
    let mut symbol: String = file_name.chars().skip(last_slash_index + 1).take(last_dot_index).collect();

    let first_dot_index = symbol.find('.').unwrap();
    symbol = symbol.chars().skip(0).take(first_dot_index).collect();
    return symbol
}

pub fn get_names_of_symbol_files() -> Vec<Company> {
    let paths = fs::read_dir("../input_data/Stocks/Test/").unwrap();
    let mut v = Vec::new();
    for path in paths {
        let mut str_file_name: String = path.unwrap().path().to_str().unwrap().to_string();
        let mut comp = Company {
            symbol: str_file_name,
        };
        //println!("Name: {}", str_file_name);
        v.push(comp);
    }
    return v
}

pub struct Company {
    symbol: String,
}
