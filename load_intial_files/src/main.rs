use std::fs;
use std::io::Write;
use std::io::{BufReader,BufRead};
use std::fs::File;

//extern crate postgres;
//use postgres::{Connection, TlsMode};

fn main() {

    // grab all the ticker names from stokcs files
    let stocks_folder_location = "../input_data/Test/";
    let symbol_file = "../output_data/stocks/symbols.csv";
    //create_symbols_file(&stocks_folder_location, &symbol_file);

    // TODO: load symbols.csv into table DIM_COMPANY
    //load_dim_company_table();

    // TODO: fact day trade
    // for each file
    //      extract the symbol
    //      is it in the symbols.csv file?
    //            NO: log
    //            YES: build a record
    //                 lookup the symbol id
    //                 lookup the date id
    //                 add the metrics to the record
    //                 spit the record to the file
    loop_thru_stock_files(stocks_folder_location, symbol_file);

}

pub fn loop_thru_stock_files(folder_location: &str, symbol_file: &str) {
    // load master list
    let mut symbols_master_list = Vec::new();
    symbols_master_list = populate_symbols_master_list(&symbol_file);

    let paths = fs::read_dir(folder_location).unwrap();
    for path in paths {
        let mut str_file_name: String = path.unwrap().path().to_str().unwrap().to_string();
        let mut symbol_from_file = extract_symbol(str_file_name);
        let mut found = false;
        for element in symbols_master_list.iter() {
            if symbol_from_file == element.symbol_name {
                // current file contains trasactions to a valid company
                println!("{} *** FOUND IT *** {}", symbol_from_file, element.symbol_name);
                found = true;
                break;
            }
        }
        if found == false {
            println!("{} ### DID NOT FIND IT IN THE MASTER FILE ###", symbol_from_file);
            // TODO: log this
        }
    }

}

pub fn populate_symbols_master_list(symbol_file: &str) -> Vec<SymbolsStruct> {
    let file = File::open(symbol_file).unwrap();
    let mut v = Vec::new();
    for line in BufReader::new(file).lines() {
        let record: String = line.unwrap();
        let first_comma_index = record.find(',').unwrap();
        let symbol_id = &record[..first_comma_index];
        let symbol_name = &record[first_comma_index+1..];
        let mut symbol_obj = SymbolsStruct {
            symbol_id: symbol_id.to_string(),
            symbol_name: symbol_name.to_string(),
        };
        v.push(symbol_obj);
    }
    return v
}

pub struct SymbolsStruct {
    symbol_id: String,
    symbol_name: String,
}

/////////////
pub fn create_symbols_file(stocks_folder_location: &str, symbol_file: &str) {

    let mut f = fs::File::create(symbol_file).expect("Unable to create file");
    let data = get_names_of_symbol_files(&stocks_folder_location);
    let mut id: i32 = 1000;
    // file  header
    f.write(b"SYMBOL_ID,SYMBOL_NAME\n").expect("Unable to write data");
    for i in  data{
        // ID
        id = id + 1;
        let mut s: String = id.to_string();
        f.write(s.as_bytes()).expect("Unable to write data");
        //Separator
        f.write(b",").expect("Unable to write data");
        // Symbol
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
    symbol = symbol.to_uppercase();
    return symbol
}

pub fn get_names_of_symbol_files(folder_location: &str) -> Vec<Company> {
    let paths = fs::read_dir(folder_location).unwrap();
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

//http://zsiciarz.github.io/24daysofrust/book/vol1/day11.html
//pub fn load_dim_company_table() {
//    //postgresql://rust:rust@localhost/rust
//    let conn = Connection::connect("postgresql://admin:Pineapple01@localhost/rose_quartz", TlsMode::None).unwrap();
//    conn.execute("CREATE TABLE person (
//                    id              SERIAL PRIMARY KEY,
//                    name            VARCHAR NOT NULL,
//                    data            BYTEA
//                  )", &[]).unwrap();
//}
