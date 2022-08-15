use std::fs::{self};
use prettytable::{Attr, color,Table, Row, Cell};

extern crate colour;
extern crate prettytable;



fn main() {

    let paths = fs::read_dir("./").unwrap();
    let mut table = Table::new();

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
        let filename = &dir.file_name().into_string().unwrap();


        let size:&str = &fs::metadata(&dir.path()).unwrap().len().to_string();
        let meta_data = &fs::metadata(&dir.path()).unwrap();
        let time_created = &fs::metadata(&dir.path()).unwrap().created()
            .unwrap().elapsed().unwrap().as_secs().to_string();
        
        
        if meta_data.is_dir(){
            table.add_row(Row::new(vec![
                Cell::new(format!("{} {}",filename,"**").as_str()),
                Cell::new(size),
                Cell::new(time_created)
            ]));
        }else{
            table.add_row(Row::new(vec![
                Cell::new(filename),
                Cell::new(size),
                Cell::new(time_created)
            ]));
        }


    }
    table.printstd();




}
