# Word List Auditor (WLA)

A tool for finding information about a given word list.

```
$ wla -G t eff.txt 
Attributes
----------
Lines found               : 7776
Mean word length          : 6.99 characters
Length of shortest word   : 3 characters (aim)
Length of longest word    : 9 characters (zoologist)
Has exact duplicates      : false
Has fuzzy duplicates      : false
Has blank lines           : false
Has start/end whitespace  : false
Has non-ASCII characters  : false
Uniform Unicode           : true
Free of prefix words?     : true
Free of suffix words?     : false
Uniquely decodable?       : true
Entropy per word          : 12.925 bits
Efficiency per character  : 1.849 bits
Assumed entropy per char  : 4.308 bits
Above brute force line?   : true
Shortest edit distance    : 1
Mean edit distance        : 6.858
Longest shared prefix     : 8
Unique character prefix   : 9
Kraft-McMillan inequality : satisfied
```

You can also pipe the output of other tools into wla: 

`tidy -L -D t eff.txt | wla`

## Formal usage/help text
```
Usage: wla [OPTIONS] [Inputted Word List]

Arguments:
  [Inputted Word List]  Word list input file

Options:
      --debug
          Print some debug information
  -g, --ignore-after <IGNORE_AFTER_DELIMITER>
          Ignore characters after the first instance of the specified delimiter until the end of line, treating anything before the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't' for tab and 's' for space. Helpful for ignoring metadata like word frequencies or dice numbers
  -G, --ignore-before <IGNORE_BEFORE_DELIMITER>
          Ignore characters before and including the first instance of the specified delimiter, treating anything after the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't' for tab and 's' for space. Helpful for ignoring metadata like word frequencies or dice numbers
  -j, --json
          Print list information in JSON format
  -s, --samples
          Print a handful of pseudorandomly selected words from the list to the terminal. Should NOT be used as actual passphrases
  -h, --help
          Print help
  -V, --version
          Print version
```

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/wla --branch main` (Run this same command to upgrade wla.)

You should then be able to run `wla --help` for help text.

Uninstall wla by running `cargo uninstall wla`.
