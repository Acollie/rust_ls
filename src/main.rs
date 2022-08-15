use std::fs::{self, Metadata};
use std::io::Error;

#[warn(unused_variables)]


#[macro_use]
extern crate colour;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};



use std::io::prelude::*;


fn main() {

    let paths = fs::read_dir("./").unwrap();
    let mut table = Table::new();
    let mut counter = 0 ;

    table.add_row(Row::new(vec![
            Cell::new("Filename")
            .with_style(Attr::ForegroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
            Cell::new("File size")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
            Cell::new("Author"),
            Cell::new("Date created").with_style(Attr::ForegroundColor(color::BLUE)),
        ]));
    for path in paths {
        let dir = path.unwrap();
        let meta_data = &dir.metadata();
        // let filename = &dir.path().to_str().unwrap();
        let filename = &dir.file_name().into_string().unwrap();
        // println!("{:?}",filename);
        
        // println!("{:?}",&dir.metadata().unwrap().permissions());
        // e_red_ln!("File: {:?}, size(types): {}, type: {:?}",&dir.path(),fs::metadata(&dir.path()).unwrap().len(),&dir.file_type().unwrap());
            // A more complicated way to add a row:

        let size:&str = &fs::metadata(&dir.path()).unwrap().len().to_string();
        let author = &fs::metadata(&dir.path()).unwrap();
        let time_created = &fs::metadata(&dir.path()).unwrap().created()
            .unwrap().elapsed().unwrap().as_secs().to_string();
        
        
        
        table.add_row(Row::new(vec![
            Cell::new(filename),
            Cell::new(size),
            Cell::new(time_created)
            
            ]));
            

        // Print the table to stdout
        counter += 1
    }
    table.printstd();

    use prettytable::{Attr, color};

/* ... */
    // let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("foobar")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("bar")
            .with_style(Attr::BackgroundColor(color::RED))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
            Cell::new("bar")
            .with_style(Attr::ForegroundColor(color::BLUE))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
        Cell::new("foo")
        ]));


}