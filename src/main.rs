use std::fs;
mod table_format;
mod file_inspection;
extern crate colour;
extern crate prettytable;
extern crate chrono;


fn main() {

    let paths = fs::read_dir("./").unwrap();

    file_inspection::loop_over_file(paths);

}
