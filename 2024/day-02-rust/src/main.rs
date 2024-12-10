use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

struct Report(Vec<i64>);

#[derive(Debug)]
struct ParseReportError;

impl Report {
    fn is_safe(&self) -> bool {
        let is_increasing = self.0[0] < self.0[1];
        let mut it = self.0.iter();
        let mut prev = it.next().unwrap();


        for current in it {
            let delta = (current - prev).abs();

            if delta < 1 || 3 < delta {
                return false;
            }

            if is_increasing && current < prev {
                return false;
            } else if (! is_increasing) && prev < current {
                return false
            }

            prev = current;
        }

        true
    }
}

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace().map(|s| s.parse().unwrap()).collect()))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("Failed to read input file");
    let reader = io::BufReader::new(file);

    let reports: Vec<Report> = reader
        .lines()
        .map(|s| s.unwrap().parse::<Report>().unwrap())
        .collect();

    let num_safe_reports = reports.into_iter().filter(|r| r.is_safe()).count();

    println!("Number of safe reports: {}", num_safe_reports);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_report_is_safe_1() {
        // Safe because the levels are all decreasing by 1 or 2.
        let report = Report::from_str("7 6 4 2 1").unwrap();
        assert!(report.is_safe());
    }

    #[test]
    fn test_report_is_safe_2() {
        // Unsafe because 2 7 is an increase of 5.
        let report = Report::from_str("1 2 7 8 9").unwrap();
        assert!(! report.is_safe());
    }

    #[test]
    fn test_report_is_safe_3() {
        // Unsafe because 6 2 is a decrease of 4.
        let report = Report::from_str("9 7 6 2 1").unwrap();
        assert!(! report.is_safe());
    }

    #[test]
    fn test_report_is_safe_4() {
        // Unsafe because 1 3 is increasing but 3 2 is decreasing.
        let report = Report::from_str("1 3 2 4 5").unwrap();
        assert!(! report.is_safe());
    }

    #[test]
    fn test_report_is_safe_5() {
        // Unsafe because 4 4 is neither an increase or a decrease.
        let report = Report::from_str("8 6 4 4 1").unwrap();
        assert!(! report.is_safe());
    }

    #[test]
    fn test_report_is_safe_6() {
        // Safe because the levels are all increasing by 1, 2, or 3.
        let report = Report::from_str("1 3 6 7 9").unwrap();
        assert!(report.is_safe());
    }
}
