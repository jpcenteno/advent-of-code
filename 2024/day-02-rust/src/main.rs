mod iterators;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use iterators::ignore_nth::IgnoreNthTrait;

use self::iterators::delta::IntoDelta;

fn sign(x: i64) -> i64 {
    if 0 < x {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

fn same_sign(a: i64, b: i64) -> bool {
    sign(a) == sign(b)
}

fn in_safe_range(delta: i64) -> bool {
    let delta = delta.abs();
    (1 <= delta) && (delta <= 3)
}

fn is_safe<'a, I: Iterator<Item = &'a i64> + Sized>(iter: I) -> bool {
    let mut deltas = iter.cloned().delta();
    let first_delta = deltas.next().unwrap();

    if ! in_safe_range(first_delta) {
        return false
    }

    for delta in deltas {
        if ! ( same_sign(first_delta, delta) && in_safe_range(delta) ) {
            return false
        }
    }

    true
}

#[derive(Debug)]
struct Report(Vec<i64>);

impl Report {
    fn is_safe(&self) -> bool {
        is_safe(self.0.iter())
    }

    fn is_safe_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        let n = self.0.len();
        for i in 0..n {
            println!("Dampening {}-th element", i);
            if is_safe(self.0.iter().ignore_nth(i)) {
                return true
            }
        }

        false
    }
}

#[derive(Debug)]
struct ParseReportError;

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

    let num_safe_reports = reports.iter().filter(|r| r.is_safe()).count();
    let num_safe_reports_dampened = reports.iter().filter(|r| r.is_safe_dampened()).count();

    println!("Number of safe reports: {}", num_safe_reports);
    println!("Number of safe reports (with dampening): {}", num_safe_reports_dampened);
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(test)]
    mod report {
        use super::*;

        #[cfg(test)]
        mod is_safe {
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

            #[test]
            fn test_report_is_safe_7() {
                // Unsafe because `2 7` is an increase of `5`.
                let report = Report::from_str("2 7 8 9").unwrap();
                assert!(! report.is_safe());
            }


            #[test]
            fn test_report_is_safe_8() {
                // Unsafe because `2 8` is an increase of `6`.
                let report = Report::from_str("1 2 8 9").unwrap();
                assert!(! report.is_safe());
            }
        }

        #[cfg(test)]
        mod is_safe_dampened {
            use super::*;

            #[test]
            fn test_report_is_safe_dampened_1() {
                // Safe without removing any level.
                let report = Report::from_str("7 6 4 2 1").unwrap();
                assert!(report.is_safe_dampened());
            }

            #[test]
            fn test_report_is_safe_dampened_2() {
                // Unsafe regardless of which level is removed.
                let report = Report::from_str("1 2 7 8 9").unwrap();
                assert!(! report.is_safe_dampened());
            }

            #[test]
            fn test_report_is_safe_dampened_3() {
                // Unsafe regardless of which level is removed.
                let report = Report::from_str("9 7 6 2 1").unwrap();
                assert!(! report.is_safe_dampened());
            }

            #[test]
            fn test_report_is_safe_dampened_4() {
                // Safe by removing the second level, 3.
                let report = Report::from_str("1 3 2 4 5").unwrap();
                assert!(report.is_safe_dampened());
            }

            #[test]
            fn test_report_is_safe_dampened_5() {
                // Safe by removing the third level, 4.
                let report = Report::from_str("8 6 4 4 1").unwrap();
                assert!(report.is_safe_dampened());
            }

            #[test]
            fn test_report_is_safe_dampened_6() {
                // Safe without removing any level.
                let report = Report::from_str("1 3 6 7 9").unwrap();
                assert!(report.is_safe_dampened());
            }
        }
    }
}
