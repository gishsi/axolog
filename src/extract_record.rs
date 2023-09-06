use crate::record::Record;

const BEGIN_DELIMITER: char = '[';
const END_DELIMITER: char = ']';

pub fn extract_record(line: &str) -> Record {
    let (line, date) = extract_date(line);
    let (line, logtype) = extract_type(line);
    let (description, cause) = extract_cause(line);

    Record::_new(logtype.to_string(), cause.to_string(), date.to_string(), description.to_string())
}

fn parse_date(date: &str) -> &str{
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
    date
}

fn  extract_token_and_line(original: &str) -> (&str, &str) {
    let begin = original.chars().position(| c | c == BEGIN_DELIMITER).unwrap();
    let end = original.chars().position(| c | c == END_DELIMITER).unwrap();
    let mut iter = original.char_indices();
    let (start, _) = iter.nth(begin).unwrap();
    let (end, _) = iter.nth(end).unwrap();
    let slice = &original[start..end];

    let removed_prefix = original.strip_prefix(slice)
        .unwrap()
        .trim();

    let line = removed_prefix;
    let token = slice.strip_prefix(BEGIN_DELIMITER).unwrap().strip_suffix(END_DELIMITER).unwrap();

    (line, token)
}

pub fn extract_date(line: &str) -> (&str, &str) {
    let (l, t) = extract_token_and_line(line);
  
    let l = parse_date(l);

    (l, t)
}

pub fn extract_type(line: &str) -> (&str, &str) {
    let (l, t) = extract_token_and_line(line);

    // parse

    (l, t)
}

pub fn extract_cause(line: &str) -> (&str, &str) {
    let (l, t) = extract_token_and_line(line);

    // parse

    (l, t)
}

#[cfg(test)]
mod extract_record_tests {
    use regex::Regex;

    use super::{extract_record, extract_date};

    #[test]
    fn extract_record_from_line() {
        let result = extract_record("[29Dec2022 02:21:19.852] [main/DEBUG] [biomes-o-plenty]: Too many chunks");

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