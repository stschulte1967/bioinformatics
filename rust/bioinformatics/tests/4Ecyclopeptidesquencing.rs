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

#[test]
#[ignore]
fn test_is_consistent() {
    let vec1: Vec<usize> = vec![0, 99, 128, 128, 147, 227, 275, 374];
    let vec2: Vec<usize> = vec![0, 1, 99, 128, 128, 128, 147, 227, 275, 374, 400];
    let result = is_consistent(vec1, vec2);
    println!("{:?}", &result);
    assert!(result);
}

#[test]
#[ignore]
fn test_expand() {
    let mut peptides = HashSet::new();
    peptides.insert(vec![]);
    println!("peptides: {:?}", expand(&peptides, CONDON_MASSES.to_vec()));
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_expand_2() {
    let mut peptides = HashSet::new();
    peptides.insert(vec![156, 114]);
    println!("peptides: {:?}", expand(&peptides, CONDON_MASSES.to_vec()));
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_expand_3() {
    let mut peptides = HashSet::new();
    peptides.insert(vec![156]);
    peptides.insert(vec![114]);
    println!("peptides: {:?}", expand(&peptides, CONDON_MASSES.to_vec()));
    assert_eq!(1,2);
}


fn test_cyclopeptide_sequencing(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4E/inputs/".to_string() + input_file);
    let input_values = input_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let output_params = read_parameters_from_file("../../data/4E/outputs/".to_string() + input_file);
    //let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = cyclopeptide_sequencing(input_values);
    println!("cyclopeptide_sequencing: {:?} {:?}", result, result.len());
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_calculate_weight() {
    let amino_acid="LEQN";
    println!("weight = {:?}", calculate_weight(amino_acid));
}

#[test]
#[ignore]
fn test_calculate_weight_2() {
    let amino_acid="L";
    println!("weight = {:?}", calculate_weight(amino_acid));
}

#[test]
#[ignore]
fn test_cyclopeptide_sequencing_1() {
    test_cyclopeptide_sequencing("testset.txt");
}

#[test]
#[ignore]
fn test_cyclopeptide_sequencing_2() {
    test_cyclopeptide_sequencing("rosalind.txt");
}

#[test]

fn test_cyclopeptide_sequencing_3() {
    test_cyclopeptide_sequencing("cogniterra.txt");
}
