use bioinformatics::{hamming_distance};
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

fn test_hamming_distance(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1G/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1G/outputs/".to_string() + input_file);
    let result = hamming_distance(&input_params[0], &input_params[1]);
    println!("{:?}", result);
    assert!(result == output_params[0].parse().unwrap());
}

#[test]
fn test1_hamming_distance() {
    test_hamming_distance("testset.txt");
}

#[test]
fn test2_hamming_distance() {
    test_hamming_distance("quiz.txt");
}

#[test]
fn test3_hamming_distance() {
    test_hamming_distance("dataset_30278_3.txt");
}