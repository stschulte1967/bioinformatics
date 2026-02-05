use bioinformatics::{gibbs_sampler, score};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_parameters_from_file(filename: String) -> Vec<String> {
    let contents = File::open(filename.clone()).expect("Failed to read file");
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

fn test_randomized_motif_search(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/2G/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/2G/outputs/".to_string() + input_file);
    
    let mut best_score = 10000;
    let mut best_motif = Vec::new();
    for i in 0..20 {
        let current_motifs = gibbs_sampler(&input_params[3..].to_vec(), input_params[0].parse().unwrap(), input_params[1].parse().unwrap(), input_params[2].parse().unwrap());
        let current_score = score(current_motifs.clone());
        if current_score < best_score {
            best_score = current_score;
            best_motif = current_motifs;
        }
    }
    println!("Results: {:?}", best_motif);
    assert_eq!(best_motif, output_params);
}

#[test]
#[ignore]
fn test1_randomized_motif_search() {
    test_randomized_motif_search("testset.txt");
}

#[test]
#[ignore]
fn test2_randomized_motif_search() {
    test_randomized_motif_search("test_1.txt");
}

#[test]
#[ignore]
fn test3_randomized_motif_search() {
    test_randomized_motif_search("test_2.txt");
}

#[test]

fn test4_randomized_motif_search() {
    test_randomized_motif_search("dataset_30309_11.txt");
}