use std::collections::HashMap;
use bioinformatics::{string_reconstruction, de_bruijn, eulerian_cycle, decomposition, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn k_universal_circular(k: usize) -> String {
    let mut patterns: Vec<String> = Vec::new();
    for i in 0..2<<(k-1) {
        patterns.push(format!("{i:0width$b}", width=k));
    }
    println!("pattern: {:?}", &patterns);
    let db = de_bruijn(patterns);
    println!("db: {:?}", &db);
    let mut path = eulerian_cycle(db);
    println!("path: {:?}", &path);
    path.pop();
    println!("path before decomposition: {:?}", &path);
    let mut d = String::new();
    for elem in path {
        if let Some(ch) = elem.chars().next() {
            d.push(ch);
        }
    }
    println!("decomposition: {:?}", &d);
    d
}

fn test_k_universal_circular(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3I/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3I/outputs/".to_string() + input_file);
    let result = k_universal_circular(input_params[0].parse().unwrap());
    println!("result = {:?}", result);
    assert_eq!(output_params[0],result);
}

#[test]
fn test_k_universal_circular_1() {
    test_k_universal_circular("testset.txt");
}

#[test]
#[ignore]
fn test_k_universal_circular_2() {
    test_k_universal_circular("rosalind.txt");
}

#[test]
#[ignore]
fn test_k_universal_circular_3() {
    test_k_universal_circular("cogniterra.txt");
}
