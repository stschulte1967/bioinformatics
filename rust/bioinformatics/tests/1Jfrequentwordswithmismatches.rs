use bioinformatics::{frequent_words_with_mismatches};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_parameters_from_file(filename: String) -> Vec<String> {
    let contents = File::open(filename).expect("Failed to read file");
    let reader = BufReader::new(contents);
    let mut results = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for entry in parts.iter() {
                    results.push(entry.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    results
}

fn have_same_elements(a: &[String], b: &[String]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut count_a = HashMap::new();
    let mut count_b = HashMap::new();

    for item in a {
        *count_a.entry(item).or_insert(0) += 1;
    }
    for item in b {
        *count_b.entry(item).or_insert(0) += 1;
    }

    count_a == count_b
}

fn test_frequent_words_with_mismatches(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1J/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1J/outputs/".to_string() + input_file);
    let result = frequent_words_with_mismatches(&input_params[0], input_params[1].parse().unwrap(),input_params[2].parse().unwrap());
    println!("{:?}", result);
    assert!(have_same_elements(&result, &output_params));
}

#[test]
fn test1_frequent_words_with_mismatches() {
    test_frequent_words_with_mismatches("testset.txt");
}

#[test]
fn test2_frequent_words_with_mismatches() {
    test_frequent_words_with_mismatches("dataset_30278_10.txt");
}