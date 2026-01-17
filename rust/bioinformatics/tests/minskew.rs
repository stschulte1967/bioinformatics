use bioinformatics::{minimum_skew};
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

fn test_minimum_skew(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1F/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/1F/outputs/".to_string() + input_file);
    let o_params: Vec<_> = output_params.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    let result = minimum_skew(&input_params[0]);
    println!("{:?} {:?}", result, o_params);
    assert!(result == o_params);
}

#[test]
fn test1_minimum_skew() {
    test_minimum_skew("testset.txt");
}

#[test]
fn test2_minimum_skew() {
    test_minimum_skew("dataset_30277_10.txt");
}

#[test]
fn test3_minimum_skew() {
    test_minimum_skew("quiz.txt");
}
