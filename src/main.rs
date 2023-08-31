use clap::Parser;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use wla::*;
// use std::env;
// use std::path::Path;
use std::path::PathBuf;
// use std::process;
pub mod display_information;
pub mod edit_distance;
use crate::display_information::display_list_information;
// pub mod input_validations;
// use crate::file_readers::*;
// use crate::file_writer::*;
// use crate::input_validations::*;
// use crate::parsers::*;

/// Combine and clean word lists
#[derive(Parser, Debug)]
#[clap(version, about, name = "tidy")]
struct Args {
    /// Do not print any extra information
    #[clap(long = "quiet")]
    quiet: bool,

    /// Word list input files. Can be more than one, in which case
    /// they'll be combined and de-duplicated. Requires at least
    /// one file.
    // #[clap(name = "Inputted Word List", required = true)]
    #[clap(name = "Inputted Word List")]
    inputted_word_list: Option<PathBuf>,
}

fn main() {
    println!("Hello, world!");
    let opt = Args::parse();
    eprintln!("Received args: {:?}", opt);

    let word_list = match opt.inputted_word_list {
        Some(path) => make_vec_from_filename(&path, None, None),
        None => read_from_stdin(),
    };
    // place holder values
    let attributes_as_json = false;
    let ignore_before_delimiter = None;
    let ignore_after_delimiter = None;
    let samples = false;

    display_list_information(
        &word_list,
        attributes_as_json,
        ignore_after_delimiter,
        ignore_before_delimiter,
        samples,
    );
}

/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>.
pub fn make_vec_from_filename(
    filename: &PathBuf,
    skip_rows_start: Option<usize>,
    skip_rows_end: Option<usize>,
) -> Vec<String> {
    let f = match File::open(filename) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file {:?}: {}", filename, e),
    };
    let file = BufReader::new(&f);

    // First, we read in all lines and call it raw_lines
    let mut raw_lines = vec![];
    for line in file.lines() {
        let l = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "Error reading a line from file {:?}: {}\nWill continue reading file.",
                    filename, e
                );
                continue;
            }
        };
        raw_lines.push(l);
    }
    // Next, we do some work incase we want to skip rows start or end
    let size_of_raw_lines = raw_lines.len();
    let mut word_list = [].to_vec();
    for (line_number, line) in raw_lines.into_iter().enumerate() {
        match (skip_rows_start, skip_rows_end) {
            (Some(skip_rows_start), Some(skip_rows_end)) => {
                if line_number >= skip_rows_start && line_number < size_of_raw_lines - skip_rows_end
                {
                    word_list.push(line);
                }
            }
            (Some(skip_rows_start), None) => {
                if line_number >= skip_rows_start {
                    word_list.push(line);
                }
            }
            (None, Some(skip_rows_end)) => {
                if line_number < size_of_raw_lines - skip_rows_end {
                    word_list.push(line);
                }
            }
            (None, None) => word_list.push(line),
        }
    }
    word_list
}

fn read_from_stdin() -> Vec<String> {
    let mut word_list = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        word_list.push(line.expect("Could not read line from standard in"));
    }
    word_list
}
