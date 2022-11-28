//! Library crate containing the common methods used by multiple days of
//! `AdventOfCode`

use std::fs::File;
use std::io::prelude::*;

/// Read the day's input data from a file.
///
/// Returns a [Result<String>](std::io::Result).
///
/// # Arguments
///
///  - `filepath` : a `&str` holding a reference to the string of the file path
///
/// # Errors
///
/// In case of I/O exception, returns an Error.
pub fn read_data(filepath: &str) -> std::io::Result<String> {
    let mut file = File::open(filepath)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string().replace('\r', ""))
}

#[macro_export]
macro_rules! test {
    ($fn:ident, 1, $exp:expr, $data:literal) => {
        #[test]
        fn $fn() {
            let data = $data;
            // The expected definitions are in separate blocks
            // So that the compiler can infer the right type from
            // The right function without us having to tell it
            let expected = $exp;
            assert_eq!(expected, solve_part_one(&data))
        }
    };
    ($fn:ident, 2, $exp:expr, $data:literal) => {
        #[test]
        fn $fn() {
            let data = $data;
            let expected = $exp;
            assert_eq!(expected, solve_part_two(&data));
        }
    }
}
