use crate::compute_attributes::decode_list;
use crate::compute_attributes::make_list_free_of_metadata;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use wla::*;
pub mod compute_attributes;
pub mod edit_distance;

/// Audit word lists
#[derive(Parser, Debug)]
#[clap(version, about, name = "wla")]
struct Args {
    /// Print some debug information
    #[clap(long = "debug")]
    debug: bool,

    /// Ignore characters after the first instance of the specified delimiter until the end of line, treating
    /// anything before the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't'
    /// for tab and 's' for space. Helpful for ignoring metadata like word frequencies or dice
    /// numbers.
    #[clap(short = 'g', long = "ignore-after")]
    ignore_after_delimiter: Option<char>,

    /// Ignore characters before and including the first instance of the specified delimiter, treating
    /// anything after the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't'
    /// for tab and 's' for space. Helpful for ignoring metadata like word frequencies or dice
    /// numbers.
    #[clap(short = 'G', long = "ignore-before")]
    ignore_before_delimiter: Option<char>,

    /// If word starts with a double quote and ends with a double quote followed by a comma, remove
    /// those 3 characters before auditing list.
    #[clap(short = 'd', long = "decode")]
    decode_words: bool,

    /// Print list information in JSON format
    #[clap(short = 'j', long = "json")]
    attributes_as_json: bool,

    /// Print a handful of pseudorandomly selected words from the list
    /// to the terminal. Should NOT be used as actual passphrases.
    #[clap(short = 's', long = "samples")]
    samples: bool,

    /// Word list input file
    #[clap(name = "Inputted Word List")]
    inputted_word_list: Option<PathBuf>,
}

fn main() -> Result<(), &'static str> {
    let opt = Args::parse();
    if opt.debug {
        eprintln!("Received args: {:?}", opt);
    }

    let word_list = match opt.inputted_word_list {
        Some(path) => make_vec_from_filename(&path, None, None),
        None => read_from_stdin(),
    };
    // Basic check of parameters
    if opt.ignore_after_delimiter.is_some() && opt.ignore_before_delimiter.is_some() {
        let error_msg = "Can't handle more than one delimiter (yet)!";
        return Err(error_msg);
    }

    let list = make_list_free_of_metadata(
        &word_list,
        opt.ignore_before_delimiter,
        opt.ignore_after_delimiter,
    );

    let list = if opt.decode_words {
        decode_list(&list)
    } else {
        list
    };

    //
    let list_attributes = make_attributes(&list);

    print_list_attributes(list_attributes, opt.attributes_as_json, opt.samples);
    Ok(())
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
