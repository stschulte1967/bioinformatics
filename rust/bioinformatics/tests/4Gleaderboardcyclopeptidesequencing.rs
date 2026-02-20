use bioinformatics::sequencing::{cyclopeptide_sequencing, cyclo_spectrum, mass, expand, calculate_weight, is_consistent};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;
use bioinformatics::common::CONDON_MASSES;

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

fn test_leaderboard_cyclopeptide_sequencing(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4E/inputs/".to_string() + input_file);
    let input_values = input_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let output_params = read_parameters_from_file("../../data/4E/outputs/".to_string() + input_file);
    //let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = leaderboard_cyclopeptide_sequencing(input_values);
    println!("cyclopeptide_sequencing: {:?} {:?}", result, result.len());
    assert_eq!(1,2);
}

#[test]

fn test_leaderboard_cyclopeptide_sequencing_1() {
    test_leaderboard_cyclopeptide_sequencing("testset.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_2() {
    test_leaderboard_cyclopeptide_sequencing("rosalind.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_3() {
    test_leaderboard_cyclopeptide_sequencing("cogniterra.txt");
}
