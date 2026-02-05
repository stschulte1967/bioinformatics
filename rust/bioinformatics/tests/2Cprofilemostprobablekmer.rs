use bioinformatics::{profile_most_probable_k_mer};
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

fn parse_profile(profile_strings: &[String], k:usize) -> Vec<HashMap<char, f32>> {

    let mut profile: Vec<HashMap<char, f32>> = vec![HashMap::new(); k];
    
    let nucleotides = ['A', 'C', 'G', 'T'];
    
    for (nucleotide_idx, nucleotide) in nucleotides.iter().enumerate() {
        let start = nucleotide_idx * k;
        let end = start + k;
        
        for (col_idx, value_str) in profile_strings[start..end].iter().enumerate() {
            if let Ok(value) = value_str.parse::<f32>() {
                profile[col_idx].insert(*nucleotide, value);
            }
        }
    }
    
    profile
}

fn test_profile_most_probable_k_mer(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/2C/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/2C/outputs/".to_string() + input_file);
    let k: usize = input_params[1].parse().unwrap();
    let profile = parse_profile(&input_params[2..], k);
    println!("{:?}", profile);
    let result = profile_most_probable_k_mer(&input_params[0], k, profile);
    println!("Result: {:?}", result);
    assert_eq!(&result, &output_params[0]);
}

#[test]
fn test1_profile_most_probable_k_mer() {
    test_profile_most_probable_k_mer("testset.txt");
}

#[test]

fn test2_profile_most_probable_k_mer() {
    test_profile_most_probable_k_mer("dataset_30305_3.txt");
}
