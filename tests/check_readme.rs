use std::fs::File;
use std::io::{BufRead, BufReader};

// Check that the example given in the README is the same as the
// one compiled in examples/hello_world.rs

#[test]
fn check_readme() {
    let readme_file = File::open("./README.md").unwrap();
    let readme_lines: Vec<_> = BufReader::new(readme_file)
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect();

    let example_row_begin = readme_lines
        .iter()
        .position(|line| line == "```rust")
        .expect("Could not find example")
        + 1;
    let example_row_end = readme_lines
        .iter()
        .position(|line| line == "```")
        .expect("Could not find example end");

    let example_file = File::open("./examples/hello_world.rs").unwrap();
    let example_lines: Vec<_> = BufReader::new(example_file)
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect();

    assert_eq!(
        readme_lines[example_row_begin..example_row_end],
        example_lines[..],
    )
}
