use bioinformatics::{median_string};
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

fn test_median_string(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/2B/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/2B/outputs/".to_string() + input_file);
    let result = median_string((input_params[1..]).to_vec(),input_params[0].parse().unwrap());
    println!("Result: {:?}", result);
    assert_eq!(&result, &output_params[0]);
}

#[test]
fn test1_median_string() {
    test_median_string("testset.txt");
}

#[test]
fn test2_median_string() {
    test_median_string("dataset_30304_9.txt");
}

#[test]
#[ignore]
fn test3_median_string() {
    test_median_string("quiz.txt");
}

