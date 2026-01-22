use bioinformatics::{approximate_pattern_count};
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

fn test_approximate_pattern_count(input_file: &str, output_file: &str) {
    let input_params = read_parameters_from_file("../../data/1I/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1I/outputs/".to_string() + output_file);
    let result = approximate_pattern_count(&input_params[1], &input_params[0], input_params[2].parse().unwrap());
    println!("{:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}

#[test]
fn test1_approximate_pattern_count() {
    test_approximate_pattern_count("testset.txt", "testset.txt");
}

#[test]
fn test2_approximate_pattern_count() {
    test_approximate_pattern_count("dataset_30278_6.txt", "dataset_30278_6.txt");
}


