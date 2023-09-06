use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use crate::commonlogtypes::CommonLogTypes;
use crate::commonlogtypes::CommonLogTypes::All;
use crate::record::Record;
use regex::Regex;

const BEGIN_DELIMITER: char = '[';
const END_DELIMITER: char = ']';

pub fn extract_records() -> Vec<Record>{
    let mut records: Vec<Record> = vec![];
    // A line represents a record or a description that belongs to the last log entry
    let record: Option<Record> = parse_line_into_record("[1] [2] [3] Des");

    match record {
        Some(r) => records.push(r),
        None => {
            let description = parse_line_into_newline_description();
            // records.last().description += '\n' + description;
        },
    }

    // There is another hidden case here: skippin/g records of unwanted log type.
    // Think we can just parse all records and then filter.

    records
}

pub fn parse_line_into_record(line: &str) -> Option<Record>{
    // if line starts with [ then it is a new record,
    // otherwise it will be a continuation of a previous record description.
    if line.starts_with(BEGIN_DELIMITER) {
        return Some(extract_record());
    }

    None
}

pub fn parse_line_into_newline_description() -> String {
    String::from("Newline description about a naughty mod!")
}

pub fn filter_records(log_type: CommonLogTypes) -> Vec<Record> {
    vec![]
}

pub fn extract_record() -> Record{
    extract_date("[99 22 3333]");
    extract_type();
    extract_cause();

    Record::default()
}

fn parse_date(date: &str) {
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
}

pub fn extract_date(line: &str) -> (&str, &str) {
    // let date = parse_date(line).to_string();
    let begin = line.chars().position(| c | c == BEGIN_DELIMITER).unwrap();
    let end = line.chars().position(| c | c == END_DELIMITER).unwrap();
    let mut iter = line.char_indices();
    let (start, _) = iter.nth(begin).unwrap();
    let (end, _) = iter.nth(end).unwrap();
    let slice = &line[start..end];

    let removed_prefix = line.strip_prefix(slice)
        .unwrap()
        .trim();

    let line = removed_prefix;

    let token = slice.strip_prefix(BEGIN_DELIMITER).unwrap().strip_suffix(END_DELIMITER).unwrap();
    
    // remove the date from the line for further processing
    // let line = line.to_string().replace(&caps[0], "").trim().to_string();

    (line, token)
}

pub fn extract_type() {

}

pub fn extract_cause() {
    
}

pub fn extract_description() {

}

#[cfg(test)]
mod parse_second_impl_tests {
    use crate::parser::extract_date;
    use regex::Regex;


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
        let result = extract_date("[29Dec2022 02:21:19.852] [main/DEBUG]");

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
;
        assert_eq!(&caps["date"], "29Dec2022 02:21:19.852");
        assert!(regex.is_match("[29Dec2022 02:21:19.852]"));
    }

}

pub fn get_record_from_line(line: &str, log_type: CommonLogTypes) -> Option<Record> {
    let mut record: Record = Record::default();
    
    let mut l = line;
    let mut i = 0;

    // todo: get rid of .unwrap()
    // todo: do this with regex instead
    while  i < 3 {
        let begin = match l.chars().position(| c | c == BEGIN_DELIMITER) {
            None => {
                panic!("Could not find the beginning delimiter in line.")
            }
            Some(b) => b
        };
        // todo: This will throw if the line passed in doesn't have the expected format, do match instead
        let end = l.chars().position(| c | c == END_DELIMITER).unwrap();

        let mut iter = l.char_indices();
        let (start, _) = iter.nth(begin).unwrap();
        let (end, _) = iter.nth(end).unwrap();
        let slice = &l[start..end];

        let removed_prefix = l.strip_prefix(slice)
            .unwrap()
            .trim();

        l = removed_prefix;

        let token = slice.strip_prefix(BEGIN_DELIMITER).unwrap().strip_suffix(END_DELIMITER).unwrap();

        // todo: error checking
        match i {
            0 => record.date = token.to_string(),
            1 => {
                let types = HashMap::from([
                    (All, "All"),
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

                if log_type == CommonLogTypes::Main ||
                    log_type == CommonLogTypes::Info ||
                    log_type == CommonLogTypes::Debug ||
                    log_type == CommonLogTypes::Warn ||
                    log_type == CommonLogTypes::Error ||
                    log_type == CommonLogTypes::Fatal {
                    if token.contains(types[&log_type]) {
                        record.m_type = token.to_string();
                    }
                }

                if log_type == All {
                    record.m_type = token.to_string();
                } else if token.to_string() != types[&log_type] && record.m_type.is_empty() {
                    return None;
                } else {
                    record.m_type = token.to_string();
                }
            },
            2 => record.cause = token.to_string(),
            _ => panic!("Too many iterations")
        }

        i += 1;
    }

    record.description = l.to_string();

    Some(record)
}

pub fn extract_records_from_file(file: fs::File, log_type: CommonLogTypes) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];
    let reader = BufReader::new(file);
    let mut newlines_between_last_log = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            #[allow(non_snake_case)]
            Error => {
                println!("Error when parsing line (possible End Of File): {}", Error.unwrap());
                return records;
            }
        };
        let is_new_entry: bool;

        if !line.starts_with('[') && newlines_between_last_log == 0 {
            is_new_entry = false;
        } else if !line.starts_with('[') && newlines_between_last_log != 0 { // line from description from a log type that user is not looking for
            continue;
        }
        else {
            newlines_between_last_log = 0;
            is_new_entry = true;

            match get_record_from_line(line.as_str(), log_type) {
                Some(r) => {
                    records.push(r);
                },
                None => {
                    newlines_between_last_log += 1;
                    continue;
                }
            };
        }

        if !is_new_entry {
            let last: &Record = match records.last() {
                None => continue,
                Some(l) => l
            };

            let last_copy = Record {
                m_type: last.m_type.to_string(),
                cause: last.cause.to_string(),
                date: last.date.to_string(),
                description: last.description.to_string() + "\n" + line.as_str(),
            };

            records.pop();
            records.push(last_copy);
        }
    }

    records
}

#[cfg(test)]
mod parser_tests {
    mod get_records_from_line {
        use crate::commonlogtypes::CommonLogTypes;
        use crate::commonlogtypes::CommonLogTypes::All;
        use crate::parser::get_record_from_line;
        #[test]
        fn unstructured_log_matches_structured_data() {
            const LOG: &str = "[29Dec2022 02:21:19.852] [main/DEBUG] [cpw.mods.modlauncher.LaunchServiceHandler/MODLAUNCHER]: Found launch services";

            let result = get_record_from_line(LOG, All).unwrap();

            assert_eq!(result.date.as_str(), "29Dec2022 02:21:19.852");
            assert_eq!(result.m_type.as_str(), "main/DEBUG");
            assert_eq!(result.cause.as_str(), "cpw.mods.modlauncher.LaunchServiceHandler/MODLAUNCHER");
        }

        #[test]
        fn type_is_main_info_result_includes_main_info() {
            const LOG: &str = "[29Dec2022 02:21:19.852] [main/INFO] [cpw.mods.modlauncher.LaunchServiceHandler/MODLAUNCHER]: Found launch services";

            let result = get_record_from_line(LOG, CommonLogTypes::MainInfo).unwrap();


            assert_eq!(result.date.as_str(), "29Dec2022 02:21:19.852");
            assert_eq!(result.m_type.as_str(), "main/INFO");
            assert_eq!(result.cause.as_str(), "cpw.mods.modlauncher.LaunchServiceHandler/MODLAUNCHER");
        }

        #[test]
        fn type_is_main_error_returns_none() {
            const LOG: &str = "[29Dec2022 02:21:19.852] [main/INFO] [cpw.mods.modlauncher.LaunchServiceHandler/MODLAUNCHER]: Found launch services";

            let result = get_record_from_line(LOG, CommonLogTypes::MainError);

            assert!(result.is_none());
        }
    }

    mod extract_records_from_file {
        use std::fs;
        use crate::commonlogtypes::CommonLogTypes::All;
        use crate::parser::extract_records_from_file;

        // todo: remove, descriptions are not part of the line parsing, we do this higher up
        // todo: I don't like this, testing is difficult, should probably remove all the record-related functionality away from "extract_records_from_file"
        #[test]
        fn bracketsindescription_lineisnotsplit() {
            let file = fs::File::open("D:/dev/untitled/example_logs/client_logs/test.log").unwrap();

            let result = extract_records_from_file(file, All);

            assert_eq!(result[0].date.as_str(), "29Dec2022 02:21:19.841");
        }
    }
}