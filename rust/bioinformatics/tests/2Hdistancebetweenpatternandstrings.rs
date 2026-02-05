use bioinformatics::{distance_between_pattern_and_strings};
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

fn test_distance_between_pattern_and_strings(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/2H/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/2H/outputs/".to_string() + input_file);
    let result = distance_between_pattern_and_strings(&input_params[0],(&input_params[1..]).to_vec());
    println!("Result: {:?}", result);
    assert_eq!(&result, &output_params[0].parse().unwrap());
}

#[test]
fn test1_distance_between_pattern_and_strings() {
    test_distance_between_pattern_and_strings("testset.txt");
}

#[test]
fn test2_distance_between_pattern_and_strings() {
    test_distance_between_pattern_and_strings("dataset_30312_1.txt");
}

