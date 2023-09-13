use std::fs;
use std::io::{BufRead, BufReader};

use crate::record::Record;

const BEGIN_DELIMITER: char = '[';
const END_DELIMITER: char = ']';

pub fn convert_lines_in_file_into_records(file: fs::File) -> Vec<Record>{
    let mut records: Vec<Record> = vec![];
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let record = match extract_record_from_line(l.as_str()) {
            Some(r) => r,
            None => add_description_to_last_record()
        };

        records.push(record);
    
    }

    records
}

pub fn add_description_to_last_record() -> Record {
    Record::default()
}

pub fn extract_record_from_line(line: &str) -> Option<Record> {
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

fn  extract_token_and_line(original: &str) -> Option<(&str, &str)> {
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

pub fn extract_date(line: &str) -> Option<(&str, &str)> {
    let data = extract_token_and_line(line);
  
    // let l = parse_date(l);

    data
}

pub fn extract_type(line: &str) -> Option<(&str, &str)> {
    let data = extract_token_and_line(line);

    // parse

    data
}

pub fn extract_cause(line: &str) -> Option<(&str, &str)> {
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
    fn extract_date_1() {
        let result = extract_date("[29Dec2022 02:21:19.852] [main/DEBUG]").unwrap();

        assert_eq!(result.0, "[main/DEBUG]");
        assert_eq!(result.1, "29Dec2022 02:21:19.852");
    }

    #[test]
    fn ansf() {
        let re = Regex::new(r"Hello (?<name>\w+)!").unwrap();
        let Some(caps) = re.captures("Hello Murphy!") else {
           println!("no match!");
          return;
        };
        println!("The name is: {}", &caps["name"]);
    }

    #[test]
    fn validate_date() { 
        let line = "29Dec2022 02:21:19.852";
        let date_format_reg = Regex::new(r"^(?<day>\d{2})(?<month>\D{3})(?<year>\d{4}) (?<hour>\d{2})(:)(?<min>\d{2})(:)(?<sec>\d{2}\..*)").unwrap();

        let Some(caps) = date_format_reg.captures(line) else { panic!("Critical failure in test: regex failed to match.") };

        println!("{}", &caps["day"]);
        println!("{}", &caps["month"]);
        println!("{}", &caps["year"]);
        println!("{}", &caps["hour"]);
        println!("{}", &caps["min"]);
        println!("{}", &caps["sec"]);

        assert_eq!(&caps["day"], "29");
        assert_eq!(&caps["month"], "Dec");
        assert_eq!(&caps["year"], "2022");
        assert_eq!(&caps["hour"], "02");
        assert_eq!(&caps["min"], "21");
        assert_eq!(&caps["sec"], "19.852");
    }

    #[test]
    fn v() {
        let line = "[29Dec2022 02:21:19.852] [main/DEBUG]";
        let regex = Regex::new(r"(^\[)(?<date>.*?)(\])").unwrap();
    
        let Some(caps) = regex.captures(line) else { panic!("Failed to match") };

        assert_eq!(&caps["date"], "29Dec2022 02:21:19.852");
        assert!(regex.is_match("[29Dec2022 02:21:19.852]"));
    }

}