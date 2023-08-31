pub mod display_information;
pub mod edit_distance;
use unicode_segmentation::UnicodeSegmentation;
/// When counting characters of a word, we want to count all accented character as 1,
/// regardless of the Unicode, to better approximate how humans would count the number
/// of characters in a word.
/// An alternate approach would be to convert each character to NFC before counting `word.nfc().count()`
/// but I don't think this handles emoji as well as grapheme cluster counting.
pub fn count_characters(word: &str) -> usize {
    word.graphemes(true).count()
}

/// Little helper function that allows users to write out whitespace
/// delimiters "s" and "t", rather than having to enter the whitespace
/// characters literally.
pub fn parse_delimiter(delimiter: char) -> Option<char> {
    if delimiter == 's' {
        Some(' ')
    } else if delimiter == 't' {
        Some('\t')
    } else {
        Some(delimiter)
    }
}

/// Simple helper function that splits a `str` by a given substring `str`,
/// Then returns a Vector of `str`s.
/// ```
/// use wla::split_and_vectorize;
/// assert_eq!(split_and_vectorize("a:b:c",":"), vec!["a","b","c"]);
/// ```
/// I find this a handy general helper function.
pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    string_to_split.split(splitter).collect()
}
