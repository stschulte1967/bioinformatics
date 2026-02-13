use std::collections::HashMap;
use bioinformatics::{k_universal_circular, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

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
