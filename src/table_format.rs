use std::fs::Metadata;
use prettytable::{Attr, color,Table, Row, Cell};

pub fn adding_row(mut table:Table, filename:&str, size:&str, meta_data:&Metadata, date_time_created: &str) ->Table{
    if meta_data.is_dir(){
        table.add_row(Row::new(vec![
            Cell::new(format!("{} {}",filename,"**").as_str()),
            Cell::new(&size),
            Cell::new(&date_time_created)
        ]));
    }else{
        table.add_row(Row::new(vec![
            Cell::new(filename),
            Cell::new(&size),
            Cell::new(&date_time_created),
        ]));
    }
    table
}


pub fn creating_table()->Table{
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
        Cell::new("Created at")
            .with_style(Attr::ForegroundColor(color::GREEN))
            .with_style(Attr::Italic(true))
            .with_hspan(1),

    ]));
    table
}