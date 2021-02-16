# How to contribute

Issues and PRs are welcome. See [CONTRIBUTING - Getting started](CONTRIBUTING.md#getting-started) if you don't know where to begin.

Before making a PR, please follow these steps:

- Run `cargo test --all-targets` and `cargo clippy --all-targets` on your code.
- Document changes in [CHANGELOG.md]. The format follows that of the druid repo:
  - Add a line at the top with `Description of your change. ([#pr-number] by [@username])`.
  - The number can be guessed or added after creating the pull request.
  - A link to the pull request must be added at the bottom of the file.
  - If you are a new contributor, please add your name to the others at the bottom of the file.

Issues don't have a specific format, but if you submit one, please be polite, and include detailed steps to reproduce.
