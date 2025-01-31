# Version 0.2.5
* Removes one binary that failed to build, halting release of v0.2.4.

# Version 0.2.4
* Upgrades `rand` dependency to v0.9.0.
* Uses latest version of `dist`/`cargo-dist` to create this release.

# Version 0.2.3

Bumps some dependencies up to current version. Also bumps `cargo-dist` version up-to-date, hopefully improving built binaries.

# Version 0.2.2

Fixes incorrect help text describing functionality of the `decode` option. Also removes a leftover debug `eprintln` statement.

# Version 0.2.1

* 4165e88 - be more explicit about types in find_mean_edit_distance function to avoid a bug when auditing long lists. See #3.

# Version 0.2.0

* dc60da3 - large reorganization and renaming of files, all to make the project easier to use as a library. Given this is technically an API change, I'm calling it a minor version change.

# Version 0.1.5

Uses a refactored and improved implementation of the Sardinas-Patterson algorithm to check lists for unique decodability. Special thanks to @westonal.
