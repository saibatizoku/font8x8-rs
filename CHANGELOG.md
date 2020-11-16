# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

## [v0.2.6] - 2020-11-15
### Added
* Test all unicode constants to check that they are sorted by unicode chars as key.

### Changed
* Fix issue #33, kind thanks to @Wallacoloo. Binary searches are performed on sorted
  FontUnicode arrays.
* `examples/unicode.rs` now prints a friendly message with the character that should
  be rendered.
* Make unicode constants public.
* Sort all unicode constants by unicode chars as key.
* Updated legacy documentation with correct unicode chars.

## [v0.2.5] - 2019-06-07
### Changed
* Made `::print_set` implementations friendlier with pulldown;
* Address with https://github.com/rust-lang/rust/issues/61478
* Standard documentation for font symbols for al sets.

## [v0.2.4] - 2018-07-31
### Added
* Add CHANGELOG.md to the crate (this document).
* Add "no-std" to 'categories' section in Cargo.toml.
* Add that the crate is compatible with "no-std" in README.md.

### Changed
* Split-off `Usage` section into legacy and unicode sub-sections in README.md.
* Re-wording of `Documentation` section with new URLs for crate docs and git repository.

## [v0.2.3] - 2018-07-22
