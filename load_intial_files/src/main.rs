use std::fs;

fn main() {
    let files_location = String::from("../input_data/stocks/Test");
    get_all_symbols(&files_location);
}


fn get_all_symbols(location: &String) {

    let paths = fs::read_dir(location).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

}
