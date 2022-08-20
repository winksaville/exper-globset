# Experiment with globset crate

The globset crate is used by [grcov](https://github.com/mozilla/grcov)
and I've seen two odd globs:
 * "/*":  at [grcov with Travis](https://github.com/mozilla/grcov#grcov-with-travis)
 * "../*", at [xtaskops](https://github.com/jondot/xtaskops/blob/51f458f95716d28dc9fa4ca09cb18493f3baec8c/xtaskops/src/tasks.rs#L70)

I added some debug to grcov and they didn't match anything.
So the first question is what do the above to regex's match.

Answers:
* "/*": Matches all absolute paths
* "../*": Matches all relative paths

Since rcov used these regexes in the "ignore" option this
means that grcov will only traverse the files in the `source-dir`.
And no file outside of the `source-dir` will be "touched".


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

