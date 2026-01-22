use bioinformatics::{approximate_pattern_positions};
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

fn test_approximate_pattern_positions(input_file: &str, output_file: &str) {
    let input_params = read_parameters_from_file("../../data/1H/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1H/outputs/".to_string() + output_file);
    let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = approximate_pattern_positions(&input_params[1], &input_params[0], input_params[2].parse().unwrap());
    println!("{:?}", result);
    assert_eq!(result, output_values);
}

#[test]
fn test1_approximate_pattern_positions() {
    test_approximate_pattern_positions("testset.txt", "testset.txt");
}

#[test]
fn test2_approximate_pattern_positions() {
    test_approximate_pattern_positions("dataset_30278_4.txt", "dataset_30278_4.txt");
}

