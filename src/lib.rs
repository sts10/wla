pub mod compute_attributes;
pub mod edit_distance;
use crate::compute_attributes::unicode_normalization_checks::uniform_unicode_normalization;
use crate::compute_attributes::uniquely_decodable::is_uniquely_decodable;
use crate::compute_attributes::*;
use unicode_segmentation::UnicodeSegmentation;

/// Caclulate all attributes of the List we're auditing
pub fn make_attributes(list: &[String]) -> ListAttributes {
    let samples = generate_samples(list);
    let shortest_word_example = list
        .iter()
        .min_by(|a, b| count_characters(a).cmp(&count_characters(b)))
        .unwrap()
        .to_string();
    let longest_word_example = list
        .iter()
        .max_by(|a, b| count_characters(a).cmp(&count_characters(b)))
        .unwrap()
        .to_string();

    let longest_shared_prefix =
        find_longest_shared_prefix(list, Some(count_characters(&longest_word_example)));

    ListAttributes {
        list_length: list.len(),
        mean_word_length: mean_word_length(list),
        entropy_per_word: calc_entropy_per_word(list.len()),
        shortest_word_length: count_characters(&shortest_word_example),
        shortest_word_example,
        longest_word_length: count_characters(&longest_word_example),
        longest_word_example,
        efficiency_per_character: efficiency_per_character(list),
        assumed_entropy_per_character: assumed_entropy_per_character(list),
        is_above_brute_force_line: is_above_brute_force_line(list),
        is_above_shannon_line: is_above_shannon_line(list),
        // new
        has_duplicates_exact: has_duplicates_exact(list),
        has_duplicates_fuzzy: has_duplicates_fuzzy(list),
        has_blank_lines: has_blank_lines(list),
        unique_words: count_unique_words(list),
        has_starting_or_trailing_space: has_starting_or_trailing_space(list),
        has_non_ascii_characters: has_non_ascii_characters(list),
        has_uniform_unicode_normalization: uniform_unicode_normalization(list),

        is_free_of_prefix_words: !has_prefix_words(list),
        is_free_of_suffix_words: !has_suffix_words(list),
        is_uniquely_decodable: is_uniquely_decodable(list),
        shortest_edit_distance: find_shortest_edit_distance(list),
        mean_edit_distance: find_mean_edit_distance(list),
        longest_shared_prefix,
        unique_character_prefix: longest_shared_prefix + 1,
        kraft_mcmillan: satisfies_kraft_mcmillan(list),
        samples,
    }
}

pub fn print_list_attributes(
    list_attributes: ListAttributes,
    attributes_as_json: bool,
    samples: bool,
) {
    if attributes_as_json {
        print_attributes_as_json(&list_attributes);
    } else {
        println!(
            "Lines found               : {}",
            list_attributes.list_length
        );
        println!(
            "Free of exact duplicates  : {}",
            !list_attributes.has_duplicates_exact
        );
        println!(
            "Free of fuzzy duplicates  : {}",
            !list_attributes.has_duplicates_fuzzy
        );
        println!(
            "Free of blank lines       : {}",
            !list_attributes.has_blank_lines
        );
        println!(
            "Unique words found        : {}",
            list_attributes.unique_words
        );
        println!(
            "No start/end whitespace   : {}",
            !list_attributes.has_starting_or_trailing_space
        );
        println!(
            "No non-ASCII characters   : {}",
            !list_attributes.has_non_ascii_characters
        );
        println!(
            "Unicode normalized        : {}",
            list_attributes.has_uniform_unicode_normalization
        );
        println!(
            "Free of prefix words      : {}",
            list_attributes.is_free_of_prefix_words
        );
        println!(
            "Free of suffix words      : {:?}",
            list_attributes.is_free_of_suffix_words
        );

        // At least for now, this one is EXPENSIVE
        println!(
            "Uniquely decodable        : {:?}",
            list_attributes.is_uniquely_decodable
        );
        println!(
            "Above brute force line    : {}",
            list_attributes.is_above_brute_force_line
        );
        //         println!(
        //             "Above Shannon line?       : {}",
        //             list_attributes.is_above_shannon_line
        //         );

        // Start of non-Bools
        println!(
            "Length of shortest word   : {} characters ({})",
            list_attributes.shortest_word_length, list_attributes.shortest_word_example
        );
        println!(
            "Length of longest word    : {} characters ({})",
            list_attributes.longest_word_length, list_attributes.longest_word_example
        );
        println!(
            "Mean word length          : {:.2} characters",
            list_attributes.mean_word_length
        );
        println!(
            "Entropy per word          : {:.3} bits",
            list_attributes.entropy_per_word
        );
        println!(
            "Efficiency per character  : {:.3} bits",
            list_attributes.efficiency_per_character
        );
        println!(
            "Assumed entropy per char  : {:.3} bits",
            list_attributes.assumed_entropy_per_character
        );
        println!(
            "Shortest edit distance    : {}",
            list_attributes.shortest_edit_distance
        );
        println!(
            "Mean edit distance        : {:.3}",
            list_attributes.mean_edit_distance
        );
        println!(
            "Longest shared prefix     : {}",
            list_attributes.longest_shared_prefix
        );
        // Numbers of characters required to definitely get to a unique
        // prefix
        println!(
            "Unique character prefix   : {}",
            list_attributes.unique_character_prefix
        );

        println!(
            "Kraft-McMillan inequality : {}",
            list_attributes.kraft_mcmillan
        );
        if samples {
            print_samples(list_attributes.samples)
        }
    }
}

fn print_attributes_as_json(list_attributes: &ListAttributes) {
    let json = serde_json::to_string(&list_attributes).unwrap();
    println!("{}", json);
}

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
