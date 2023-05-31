use std::fs::{ReadDir};
use std::fs;
use chrono::offset::Utc;
use chrono::DateTime;
use crate::table_format;

pub fn loop_over_file(paths:ReadDir){
    let mut table = table_format::creating_table();
    for path in paths {
        let dir = path.unwrap();
        let filename = &dir.file_name().into_string().unwrap();


        let size:&str = &fs::metadata(&dir.path()).unwrap().len().to_string().to_owned();
        let meta_data = &fs::metadata(&dir.path()).unwrap();
        let time_created = fs::metadata(&dir.path()).unwrap().created()
            .unwrap();
        let datetime: DateTime<Utc> = time_created.into();
        let date_time_created = datetime.format("%d/%m/%Y %T").to_string();
        table_format::adding_row(& mut table,filename.as_str(),size,date_time_created.as_str(),file_type(meta_data));
    
    }
    table.printstd();
}

fn file_type(meta_data:&fs::Metadata)->&str{
    if meta_data.is_dir(){
        return "Directory"
    }
    else if meta_data.is_file(){
        return "File"
    }
    else{
        return "Unknown"
    }
}

