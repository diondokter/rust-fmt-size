#!/usr/bin/env cargo

//! ```cargo
//! [dependencies]
//! itertools = "0.11"
//! ```

use itertools::Itertools;
use std::fmt::Write;
use std::process::Command;

fn main() {
    let mut output_file_contents = String::from("# Output\n\n");

    output_file_contents += "## Fmt-comparison\n\n";
    output_file_contents += "### Fmt\n\nTest of builtin fmt machinery.\n";
    output_file_contents += &run_tests(
        "./fmt-comparison",
        &["raw", "fmt-no-args", "fmt-u32", "fmt-i32", "fmt-f32"],
    );
    output_file_contents += "\n";

    output_file_contents += "### Ufmt\n\nTest of the ufmt crate.\n\n*NOTE:* The f32 implementation has many limitations.\n";
    output_file_contents += &run_tests(
        "./fmt-comparison",
        &["raw", "ufmt-no-args", "ufmt-u32", "ufmt-i32", "ufmt-f32"],
    );
    output_file_contents += "\n";

    output_file_contents += "## Dyn-comparison\n\nSee that `raw` is similar to `ufmt-no-args` and `dyn` is similar to `fmt-no-args` of the fmt-comparison test.\n";
    output_file_contents += &run_tests("./dyn-comparison", &["raw", "dyn"]);
    output_file_contents += "\n";

    std::fs::write("results.md", &output_file_contents).unwrap();
}

fn run_tests(project: &str, features: &[&str]) -> String {
    let mut output_table = format!(
        r#"
|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
"#,
    );

    let mut results = Vec::new();

    for feature in Some(vec![String::from("")])
        .into_iter()
        .chain(
            features
                .into_iter()
                .map(|feature| String::from(*feature))
                .combinations(1),
        )
        .chain(
            features
                .into_iter()
                .map(|feature| String::from(*feature))
                .combinations(2),
        )
        .chain(Some(vec![features.join(",")]))
    {
        let feature = feature.join(",");

        let output = Command::new("cargo")
            .args([
                "size",
                "--release",
                "--features",
                &feature,
                "--",
                "--format=sysv",
            ])
            .current_dir(project)
            .output()
            .expect("failed to execute process");
        let output_text = String::from_utf8_lossy(&output.stdout);
        let result = SizeResult::new(feature.into(), &output_text);
        println!("{result:?}");
        results.push(result);
    }

    results.sort_by_key(|result| result.text + result.rodata);

    for result in results {
        writeln!(
            output_table,
            "|{}|{}|{}|{}|",
            result.features,
            result.text,
            result.rodata,
            result.text + result.rodata
        )
        .unwrap();
    }

    output_table
}

#[derive(Debug)]
struct SizeResult {
    features: String,
    text: u32,
    rodata: u32,
}

impl SizeResult {
    fn new(features: String, output: &str) -> Self {
        let mut text = u32::MAX;
        let mut rodata = u32::MAX;

        for line in output.lines() {
            if line.starts_with(".text") {
                text = line
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            if line.starts_with(".rodata") {
                rodata = line
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }

        Self {
            features,
            text,
            rodata,
        }
    }
}
