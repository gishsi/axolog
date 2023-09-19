use std::fs::File;
use std::io::Write;
use clap::Parser;
use commonlogtypes::CommonLogTypes;
use converter::{convert_lines_into_records, filter_records_by_type};
use crate::args::Args;
use crate::record::Record;

mod args;
mod commonlogtypes;
mod record;
mod converter;

fn main() {
    let args = args::Args::parse();

    convert_log_file(args);
}

/// Parses the unstructured Minecraft log file pointed to by the user into structured JSON equivalent.
///
/// # Panics
///
/// Panics if log file poitned to does not exist, path to save the JSON file with structured logs does not exist or 
/// critical failure resulting from a corrupt log file occured.
pub fn convert_log_file(args: Args) {
    let Ok(file) = File::open(args.path_to_file) else {
        panic!("Erroroccured  when opening file.");
    };

    let mut records: Vec<Record> = convert_lines_into_records(file);

    if args.log_type != CommonLogTypes::All {
        records = filter_records_by_type(args.log_type, &mut records);
    }
    

    if !args.path_to_save.is_empty() {
        match save_to_file(records, args.path_to_save.as_str()) {
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
    for extracted_record in records {
        println!("{:?}", extracted_record);
    }
}

fn save_to_file(records: Vec<Record>, path: &str) -> Result<bool, &str> {
    let Ok(json) = serde_json::to_string_pretty(&records) else {
        panic!("Error occured when saving the file, path: {}", path);
    };

    let Ok(mut formatted_data_file) = File::create(path) else {
        panic!("Error occured when creating a file, path: {}", path);
    };

    return match formatted_data_file.write_all(json.as_ref()) {
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            panic!("Error occured when writing to the file\n{}\npath: {}", e, path);
        }
    };
}