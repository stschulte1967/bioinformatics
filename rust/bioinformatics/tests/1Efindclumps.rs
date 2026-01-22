use bioinformatics::{find_clumps};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

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


fn test_find_clumps(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1E/inputs/".to_string() + input_file);
    let output_params: HashSet<String> = read_parameters_from_file("../../data/1E/outputs/".to_string() + input_file).into_iter().collect();
    let result = find_clumps(&input_params[0], input_params[1].parse().unwrap(), input_params[2].parse().unwrap(), input_params[3].parse().unwrap());
    println!("{:?}", result);
    assert!(result == output_params);
}

fn test_find_clumps_e_coli(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/1E/inputs/".to_string() + input_file);
    let output_params: HashSet<String> = read_parameters_from_file("../../data/1E/outputs/".to_string() + input_file).into_iter().collect();
    let result = find_clumps(&input_params[0], input_params[1].parse().unwrap(), input_params[2].parse().unwrap(), input_params[3].parse().unwrap());
    let num_results = result.len();
    println!("{:?}", result);
    assert!(num_results == output_params.iter().next().expect("no elements").parse().unwrap());
}

#[test]
fn test1_find_clumps() {
    test_find_clumps("testset.txt");
}

#[test]
fn test2_find_clumps() {
    test_find_clumps("dataset_30274_5.txt");
}

#[test]
#[ignore]
fn test3_find_clumps() {
    test_find_clumps_e_coli("E_coli_set.txt");
}
