use prettytable::{Attr, color,Table, Row, Cell};

pub fn adding_row(table:& mut Table, filename:&str, size:&str, date_time_created: &str, file_type: &str){

    table.add_row(Row::new(vec![
        Cell::new(filename),
        Cell::new(&size),
        Cell::new(&file_type),
        Cell::new(&date_time_created),
    ]));
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
        Cell::new("File Type")
            .with_style(Attr::ForegroundColor(color::WHITE))
            .with_style(Attr::Italic(true))
            .with_hspan(1),
        Cell::new("Created at")
            .with_style(Attr::ForegroundColor(color::GREEN))
            .with_style(Attr::Italic(true))
            .with_hspan(1),




    ]));
    table
}