use bioinformatics::{debruijn};
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

fn test_debruijn(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3D/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3D/outputs/".to_string() + input_file);
    let result = debruijn(input_params[1].clone(), input_params[0].parse().unwrap());
    println!("{:?}", result);
    assert_eq!(1,2);
    //assert!(have_same_elements(&result, &output_params));
}

#[test]
#[ignore]
fn test1_debruijn() {
    test_debruijn("testset.txt");
}

#[test]

fn test2_debruijn() {
    test_debruijn("dataset_30123_1.txt");
}

#[test]
#[ignore]
fn test3_debruijn() {
    test_debruijn("input_1.txt");
}