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


/// checks whether all elements of vector seq are in vector with.
/// Assumes that seq and with are ordered!
fn is_consistent(seq: Vec<usize>, with: Vec<usize>) -> bool {
    let mut current_pos_in_with = 0;
    let mut consistent = true;
    'outher: for elem in &seq {
        while current_pos_in_with < with.len() {
            if elem == &with[current_pos_in_with] {
                println!("elem: {:?}, elem2: {:?}", elem, with[current_pos_in_with]);
                current_pos_in_with += 1;
                continue 'outher;
            } else {
                println!("elem: {:?}, elem2: {:?}", elem, with[current_pos_in_with]);
                current_pos_in_with += 1;
            }
        }
        consistent = false;
        break;
    }
    consistent
}

#[test]
fn test_is_consistent() {
    let vec1: Vec<usize> = vec![0, 99, 128, 147, 227, 275, 374];
    let vec2: Vec<usize> = vec![0, 1, 99, 128, 147, 227, 275, 374, 400];
    println!("{:?}", is_consistent(vec1, vec2));
    assert_eq!(1,2);
}
/*
fn test_cyclopeptide_sequencing(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4C/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4C/outputs/".to_string() + input_file);
    let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = cyclopeptide_sequencing(&input_params[0]);
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
fn test_cyclopeptide_sequencing_1() {
    test_cyclopeptide_sequencing("testset.txt");
}

#[test]
fn test_cyclopeptide_sequencing_2() {
    test_cyclopeptide_sequencing("rosalind.txt");
}

#[test]
fn test_cyclopeptide_sequencing_3() {
    test_cyclopeptide_sequencing("cogniterra.txt");
}
*/