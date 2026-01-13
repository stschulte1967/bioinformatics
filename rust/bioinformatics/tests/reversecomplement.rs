use bioinformatics::{reverse_complement};
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

fn test_reverse_complement(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1C/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1C/outputs/".to_string() + input_file);
    let result = reverse_complement(&input_params[0]);
    assert_eq!(result, output_params[0]);
}

#[test]
fn test1_reverse_complement() {
    test_reverse_complement("testset.txt");
}

#[test]
fn test2_reverse_complement() {
    test_reverse_complement("reverseComplement.txt");
}