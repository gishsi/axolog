use std::fs::File;
use std::io::Write;
use clap::{Parser};
use crate::args::Args;
use crate::parser::extract_records_from_file;
use crate::record::Record;

mod args;
mod commonlogtypes;
mod parser;
mod record;

fn main() {
    let args = args::Args::parse();

    analyze_log(args);
}

pub fn analyze_log(args: Args) {
    let file = match File::open(args.path_to_file) {
        Ok(f) => f,
        Err(err) => {
            panic!("Unknown error when opening file. \n\t{}", err);
        }
    };

    let extracted_records: Vec<Record> = extract_records_from_file(file, args.log_type);

    if !args.path_to_save.is_empty() {
        match save_to_file(extracted_records, args.path_to_save.as_str()) {
            Ok(_) => {
                println!("Logs parsed and saved at {}", args.path_to_save.as_str());
                return;
            }
            Err(err) => {
                panic!("Failure when saving to file with {:?}", err);
            }
        }
    }

    // If path to save is empty, or invalid print the parsed records into the cli
    for extracted_record in extracted_records {
        println!("{:?}", extracted_record);
    }
}

fn save_to_file(records: Vec<Record>, path: &str) -> Result<bool, &str> {
    let json = match serde_json::to_string_pretty(&records) {
        Ok(j) => j,
        Err(e) => {
            panic!("{}\npath: {}", e, path);
        }
    };

    let mut formatted_data_file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}\npath: {}", e, path);
        }
    };

    return match formatted_data_file.write_all(json.as_ref()) {
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            // todo: why not red output from panic!()?
            panic!("{}\npath: {}", e, path);
        }
    };
}