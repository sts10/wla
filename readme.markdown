# Word List Auditor (WLA)

A tool for finding information about a given word list.

## Examples
```
$ wla -G t -s eff_long.txt 
Lines found               : 7776
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 7776
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : true
Free of suffix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (aim)
Length of longest word    : 9 characters (zoologist)
Mean word length          : 6.99 characters
Entropy per word          : 12.925 bits
Efficiency per character  : 1.849 bits
Assumed entropy per char  : 4.308 bits
Shortest edit distance    : 1
Mean edit distance        : 6.858
Longest shared prefix     : 8
Unique character prefix   : 9
Kraft-McMillan inequality : satisfied

Word samples
------------
chivalry external drift bobsled tacking brilliant
outsmart liquid unhealthy equator request deafening
clamshell arguable excusably morally keg sprout
preseason snide unranked crummiest absinthe gummy
wages paper coastland unbridle zesty chitchat
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
  -d, --decode
          If word starts with a double quote and ends with a double quote, remove those 3 characters before auditing list
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

### Latest release
Check [the GitHub releases page](https://github.com/sts10/wla/releases) for binaries of the latest released version of WLA.

### Using Rust and Cargo
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/wla --locked --branch main` (Run this same command to upgrade wla.) Uninstall wla by running `cargo uninstall wla`.

Once installed, you should then be able to run `wla --help` for help text.

## About the information reported by this program

### Prefix codes, suffix codes, and uniquely decodable codes

If a word list is "uniquely decodable" that means that words from the list can be safely combined _without_ a delimiter between each word, e.g. `enticingneurosistriflecubeshiningdupe`.

As a brief example, if a list has "boy", "hood", and "boyhood" on it, users who specified they wanted two words worth of randomness (entropy) might end up with "boyhood", which an attacker guessing single words would try. Removing the word "boy", which makes the remaining list uniquely decodable, prevents this possibility from occurring.

WLA can check if a list is free of [prefix words](https://en.wikipedia.org/wiki/Prefix_code) and if it is free of suffix words. 

It also checks if a list is uniquely decodable. It does this using [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm). 

## On maximum shared prefix length

If WLA reports that the shared prefix length is say, 4, that means that knowing the first 4 characters of any word on the generated list is sufficient to know which word it is.

On this hypothetical list, we'd know that if a word starts with "radi", we know it must be the word "radius".

## What is "Efficiency per character" and "Assumed entropy per char" and what's the difference?

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the **average** word length of words on the list, we get a value we might call "efficiency per character". This just means that, on average, you get _E_ bits per character typed.

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the length of the **shortest** word on the list, we get a value we might call "assumed entropy per char" (or character).

For example, if we're looking at the EFF long list, we see that it is 7,776-words long, so we'd assume an entropy of log<sub>2</sub>7776 or 12.925 bits per word. The average word length is 7.0, so the efficiency is 1.8 bits per character. (I got this definition of "efficiency" from [an EFF blog post about their list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).) And lastly, the shortest word on the list is three letters long, so we'd divide 12.925 by 3 and get an "assumed entropy per character" of about 4.31 bits per character.

I contend that this "assumed entropy per character" value in particular may be useful when we ask the more theoretical question of "how short should the shortest word on a good word list should be?" There may be an established method for determining what this minimum word length should be, but if there is I don't know about it yet! Here's the math I've worked out on my own.

### The "brute force line"

Assuming the list is comprised of 26 unique characters, if the shortest word on a word list is shorter than log<sub>26</sub>(list_length), there's a possibility that a user generates a passphrase such that the formula of entropy_per_word = log<sub>2</sub>(list_length) will _overestimate_ the entropy per word. This is because a brute-force character attack would have fewer guesses to run through than the number of guesses we'd assume given the word list we used to create the passphrase.

As an example, let's say we had a 10,000-word list that contained the one-character word "a" on it. Given that it's 10,000 words, we'd expect each word to add an additional ~13.28 bits of entropy. That would mean a three-word passphrase would give users 39.86 bits of entropy. However! If a user happened to get "a-a-a" as their passphrase, a brute force method shows that entropy to be only 14.10 bits (4.7 \* 3 words). Thus we can say that it falls below the "brute force line", a phrase I made up.

#### Maximum word list lengths to clear the Brute Force Line

Formula:

Where _S_ is the length of the shortest word on the list, 26 is the number of letters in the English alphabet, and _M_ is max list length: _M_ = 2<sup>_S_ * log<sub>2</sub>(26)</sup>. Conveniently, this simplifies rather nicely to _M_ = 26<sup>_S_</sup>.

(or in Python: `max_word_list_length = 26**shortest_word_length`)

| shortest word length | max list length |
|----------------------|-----------------|
| 2                    | 676             |
| 3                    | 17576           |
| 4                    | 456976          |
| 5                    | 11881376        |

## How WLA counts the length of a word

When counting the length of a word, WLA counts the number of [grapheme clusters](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries) in the word. Generally, less common characters like accented letters and emoji all count as 1 grapheme cluster and thus, to WLA, one character. I believe this better fits with how us humans intuitively count characters in a string/word.

## What types of files does WLA work with?
In general, WLA expects inputted files to have one word per line. 

### Line endings
WLA supports `\n` and `\r\n` line endings.


## For developers: How to create a release

This project uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) to create releases. 

Some of [my personal docs are here](https://sts10.github.io/docs/cargo-dist-tips.html); but basically, `cargo install cargo-dist`. When you're ready to cut a new release, test the current state of the project with `cargo dist build` and `cargo dist plan`. If that went well, create a new git tag that matches the current project version in `Cargo.toml` with `git tag vX.X.X`. Finally, run `git push --tags` to kick off the release process. GitHub will handle it from here -- check your GitHub Releases page in about 5 to 10 minutes.
