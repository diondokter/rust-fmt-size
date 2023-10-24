#!/usr/bin/env cargo

//! ```cargo
//! [dependencies]
//! itertools = "0.11"
//! ```

use itertools::Itertools;
use std::fmt::Write;
use std::process::Command;

fn main() {
    let features = ["raw", "fmt-no-args", "fmt-u32", "fmt-i32", "fmt-f32"];

    let mut output_file_contents = String::from(
        r#"# Output

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
"#,
    );

    let mut results = Vec::new();

    for feature in Some(vec![""])
        .into_iter()
        .chain(features.into_iter().combinations(1))
        .chain(features.into_iter().combinations(2))
        .chain(features.into_iter().combinations(3))
        .chain(features.into_iter().combinations(4))
        .chain(features.into_iter().combinations(5))
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
            .current_dir("./test-project")
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
            output_file_contents,
            "|{}|{}|{}|{}|",
            result.features,
            result.text,
            result.rodata,
            result.text + result.rodata
        )
        .unwrap();
    }

    std::fs::write("results.md", &output_file_contents).unwrap();
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
