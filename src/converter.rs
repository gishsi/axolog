use core::panic;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::commonlogtypes::CommonLogTypes;
use crate::record::Record;

/// Each new log entry begins with this character
const BEGIN_DELIMITER: char = '[';

// For each entry there are four main pieces of information. This character marks the end of each one (expect the description)
const END_DELIMITER: char = ']';


/// Each line in a valid log file can be either a new entry, or can hold additional information about the previous entry 
///     (newline, overflow description).
pub fn convert_lines_into_records(file: fs::File) -> Vec<Record>{
    let mut records: Vec<Record> = vec![];
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let Some(record) = extract_record_from_line(line.as_str()) else {
            add_description_to_last_record(&mut records, line);
            continue;
        };

        records.push(record);
    }

    records
}

/// Support for filtering by the most common log types observed.
pub fn filter_records_by_type(log_type: CommonLogTypes, records: &mut Vec<Record>) -> Vec<Record> {
    let result: Vec<Record> = records
        .drain(..)
        .filter(|record| {
            let types = HashMap::from([
                (CommonLogTypes::All, "All"),
                (CommonLogTypes::Main, "main"),
                (CommonLogTypes::Info, "INFO"),
                (CommonLogTypes::Debug, "DEBUG"),
                (CommonLogTypes::Warn, "WARN"),
                (CommonLogTypes::Error, "ERROR"),
                (CommonLogTypes::Fatal, "FATAL"),
                (CommonLogTypes::MainInfo, "main/INFO"),
                (CommonLogTypes::MainDebug, "main/DEBUG"),
                (CommonLogTypes::MainError, "main/ERROR"),
                (CommonLogTypes::MainWarn, "main/WARN"),
            ]);

            if (log_type == CommonLogTypes::Main ||
                    log_type == CommonLogTypes::Info ||
                    log_type == CommonLogTypes::Debug ||
                    log_type == CommonLogTypes::Warn ||
                    log_type == CommonLogTypes::Error ||
                    log_type == CommonLogTypes::Fatal) && record.m_type.contains(types[&log_type]) {
                        return true;
                }

            // to_string ownes this value just for comparison
            record.m_type == *types[&log_type]
        })
        .collect();

    result
}

/// Adds the overflow (newline) description onto the last added record. 
/// 
/// # Panics
/// 
/// Panics if there are no records - file must be corrupted.
fn add_description_to_last_record(records: &mut [Record], line: String) {
    let Some(last) = records.last_mut() else {
        panic!("Could not find a record to add the description to.");
    };

    last.description = last.description.to_string() + "\n" + line.as_str();
}

/// Each entry contains four pieces of data: date, type of the log, what caused it (which thread), and description. 
/// First three are contained within a set of square brackets (e.g. [29Dec2022 02:21:19.852]). Description is specified after the cause,
/// separated by a colon.
fn extract_record_from_line(line: &str) -> Option<Record> {
    let Some((line, date)) = extract_date(line) else {
        return None;
    };

    let Some((line, logtype)) = extract_type(line) else {
        return None;
    };

    let Some((description, cause)) = extract_cause(line) else {
        return None;
    };

    Some(Record::_new(logtype.to_string(), cause.to_string(), date.to_string(), description.to_string()))
}

/// Parses the date into the format used by the Minecraft loggger.
fn parse_date(date: &str) -> Option<&str> {
    // Extract the date
    // let regex = Regex::new(r"(^\[)(?<date>.*?)(\])")
    //     .unwrap();
    
    // let Some(caps) = regex.captures(date) else { 
    //     panic!("Failed to match") 
    // };

    // // Validate the format
    // let date_format_reg = Regex::new(r"^(?<day>\d{2})(?<month>\D{3})(?<year>\d{4}) (?<hour>\d{2})(:)(?<min>\d{2})(:)(?<sec>\d{2}\..*)")
    //     .unwrap();

    // let Some(c) = date_format_reg.captures(&caps["date"]) else { 
    //     panic!("Critical failure in test: regex failed to match.") 
    // };

    // &c[0]
    // let c = &caps[0].to_owned();

    // c
    Some(date)
}

/// A token can be a date, type, or a cause. The line returned is the remaining data after removing the token with corresponding set of brackets.
fn  extract_token_and_line(original: &str) -> Option<(&str, &str)> {
    // https://stackoverflow.com/questions/55755552/what-is-the-rust-equivalent-to-a-try-catch-statement
    let Some(begin) = original.chars().position(| c | c == BEGIN_DELIMITER) else {
        dbg!("Could not find the beginning delimeter: {}", original);
        return None;
    };

    let Some(end) = original.chars().position(| c | c == END_DELIMITER) else {
        dbg!("Could not find the ending delimeter: {}", original);
        return None;
    };

    let mut iter = original.char_indices();

    let Some((start, _)) = iter.nth(begin) else {
        dbg!("Failed to find the index of the beginning delimeter: {}", original);
        return None;
    };

    let Some((end, _)) = iter.nth(end) else {
        dbg!("Failed to find the index of the ending delimeter: {}", original);
        return None;
    };

    let slice = &original[start..end];

    let Some(removed_prefix) = original.strip_prefix(slice) else {
        dbg!("Failed to remove the prefix from line: {}", original);
        return None;
    };

    let removed_prefix = removed_prefix.trim();

    let line = removed_prefix;

    let Some(token_suffix_removed) = slice.strip_prefix(BEGIN_DELIMITER) else {
        dbg!("Failed to remove the begin delimeter: {}", original);
        return None;
    };

    let Some(token) = token_suffix_removed.strip_suffix(END_DELIMITER) else {
        dbg!("Failed to remove the end delimeter: {}", original);
        return None;
    };

    Some((line, token))
}

/// Extracts date from the provided line, parsers it and returns the date and the remaining line if valid. 
/// Must be called first when analyzing a log entry.
fn extract_date(line: &str) -> Option<(&str, &str)> {
    let data = extract_token_and_line(line);
  
    // let l = parse_date(l);

    data
}

/// Extracts type from the provided line and returns the type and the remaining line if valid. 
/// Must be called second when analyzing a log entry, after extracting the date (see [extract_date()]).
fn extract_type(line: &str) -> Option<(&str, &str)> {
    let data = extract_token_and_line(line);

    // parse

    data
}

/// Extracts cause from the provided line and returns the cause and the remaining line if valid. 
/// Must be called last when analyzing a log entry, after extracting the date (see [extract_date()]) and the type (see [extract_type()]).
fn extract_cause(line: &str) -> Option<(&str, &str)> {
    let data = extract_token_and_line(line);

    // parse

    data
}

#[cfg(test)]
mod extract_record_tests {
    use regex::Regex;

    use super::{extract_record_from_line, extract_date};

    #[test]
    fn extract_record() {
        let result = extract_record_from_line("[29Dec2022 02:21:19.852] [main/DEBUG] [biomes-o-plenty]: Too many chunks").unwrap();

        assert_eq!(result.date, "29Dec2022 02:21:19.852");
        assert_eq!(result.m_type, "main/DEBUG");
        assert_eq!(result.cause, "biomes-o-plenty");
        assert_eq!(result.description, ": Too many chunks");
    }

    #[test]
    fn remove_substring() {
        let line = "[29Dec2022 02:21:19.852] [main/DEBUG]";
        let subline = "[29Dec2022 02:21:19.852]";

        println!("Does line contains sub? {}", line.to_string().contains(subline));

        let line = line.replace(subline, "");

        println!("Removed? {}", line);

        assert_eq!(line, " [main/DEBUG]");
        assert_eq!(line.trim(), "[main/DEBUG]");
    }

    #[test]
    fn extract_date_() {
        let result = extract_date("[29Dec2022 02:21:19.852] [main/DEBUG]").unwrap();

        assert_eq!(result.0, "[main/DEBUG]");
        assert_eq!(result.1, "29Dec2022 02:21:19.852");
    }

    #[test]
    fn validate_date_regex() { 
        let line = "29Dec2022 02:21:19.852";
        let date_format_reg = Regex::new(r"^(?<day>\d{2})(?<month>\D{3})(?<year>\d{4}) (?<hour>\d{2})(:)(?<min>\d{2})(:)(?<sec>\d{2}\..*)").unwrap();

        let Some(caps) = date_format_reg.captures(line) else { panic!("Critical failure in test: regex failed to match.") };

        assert_eq!(&caps["day"], "29");
        assert_eq!(&caps["month"], "Dec");
        assert_eq!(&caps["year"], "2022");
        assert_eq!(&caps["hour"], "02");
        assert_eq!(&caps["min"], "21");
        assert_eq!(&caps["sec"], "19.852");
    }
}