use bioinformatics::{pattern_count};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn test_pattern_count(input_file: &str, output_file: &str) {
    let input_params = read_parameters_from_file("../../data/1A/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1A/outputs/".to_string() + output_file);
    let output_value: usize = output_params[0].parse().unwrap();
    let result = pattern_count(&input_params[0], &input_params[1]);
    assert_eq!(result, output_value);
}

#[test]
fn test1_pattern_count() {
    test_pattern_count("input_1.txt", "output_1.txt");
}

#[test]
fn test2_pattern_count() {
    test_pattern_count("input_2.txt", "output_2.txt");
}

#[test]
fn test3_pattern_count() {
    test_pattern_count("input_3.txt", "output_3.txt");
}

#[test]
fn test4_pattern_count() {
    test_pattern_count("input_4.txt", "output_4.txt");
}

#[test]
fn test5_pattern_count() {
    test_pattern_count("input_5.txt", "output_5.txt");
}

#[test]
fn test6_pattern_count() {
    test_pattern_count("input_6.txt", "output_6.txt");
}

#[test]
fn test7_pattern_count() {
    test_pattern_count("input_7.txt", "output_7.txt");
}

#[test]
fn test8_pattern_count() {
    test_pattern_count("input_8.txt", "output_8.txt");
}

#[test]
fn test9_pattern_count() {
    test_pattern_count("dataset_30272_6.txt", "output_dataset.txt");
}