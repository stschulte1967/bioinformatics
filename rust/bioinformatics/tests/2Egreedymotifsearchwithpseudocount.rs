use bioinformatics::{greedy_motif_search_with_pseudocount};
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

fn test_greedy_motif_search_with_pseudocount(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/2E/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/2E/outputs/".to_string() + input_file);
    let result = greedy_motif_search_with_pseudocount(input_params[0].parse().unwrap(), input_params[1].parse().unwrap(), input_params[2..].to_vec());
    println!("Result: {:?}", result);
    assert_eq!(&result, &output_params);
}

#[test]
fn test1_greedy_motif_search_with_pseudocount() {
    test_greedy_motif_search_with_pseudocount("testset.txt");
}

#[test]
fn test2_greedy_motif_search_with_pseudocount() {
    test_greedy_motif_search_with_pseudocount("dataset_30306_9.txt");
}

