use bioinformatics::sequencing::{theoretical_spectrum, calculate_weight};
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

fn test_theoretical_spectrum(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4C/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4C/outputs/".to_string() + input_file);
    let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = theoretical_spectrum(&input_params[0]);
    println!("{:?}", result);
    assert_eq!(&result, &output_values);
}

#[test]
fn test_calculate_weight() {
    let amino_acid="LEQN";
    println!("weight = {:?}", calculate_weight(amino_acid));
}

#[test]
fn test_calculate_weight_2() {
    let amino_acid="L";
    println!("weight = {:?}", calculate_weight(amino_acid));
}

#[test]
fn test_theoretical_spectrum_1() {
    test_theoretical_spectrum("testset.txt");
}

#[test]
fn test_theoretical_spectrum_2() {
    test_theoretical_spectrum("rosalind.txt");
}

#[test]
fn test_theoretical_spectrum_3() {
    test_theoretical_spectrum("cogniterra.txt");
}