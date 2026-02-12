use std::collections::HashMap;
use bioinformatics::{string_reconstruction, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn test_string_reconstruction(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3H/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3H/outputs/".to_string() + input_file);
    let result = string_reconstruction(input_params[0].parse().unwrap(), input_params[1..].to_vec());
    println!("result1 = {:?}", result);
    assert_eq!(output_params[0],result);
}

#[test]
fn test_string_reconstruction_1() {
    test_string_reconstruction("testset.txt");
}

#[test]
fn test_string_reconstruction_2() {
    test_string_reconstruction("rosalind.txt");
}

#[test]
fn test_string_reconstruction_3() {
    test_string_reconstruction("cogniterra.txt");
}
