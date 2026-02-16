use std::collections::HashMap;
use bioinformatics::{string_from_paired_composition, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn test_string_from_paired_composition(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3J/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3J/outputs/".to_string() + input_file);
    let result = string_from_paired_composition(input_params[0].parse().unwrap(),
        input_params[1].parse().unwrap(),
        &input_params[2..].to_vec());
    //println!("result1 = {:?}", result);
    assert_eq!(output_params[0],result);
}

#[test]
#[ignore]
fn test_string_from_paired_composition_1() {
    test_string_from_paired_composition("testset.txt");
}

#[test]
#[ignore]
fn test_string_from_paired_composition_2() {
    test_string_from_paired_composition("rosalind.txt");
}

#[test]
#[ignore]
fn test_string_from_paired_composition_3() {
    test_string_from_paired_composition("cogniterra.txt");
}

#[test]
fn test_string_from_paired_composition_4() {
    test_string_from_paired_composition("quiz.txt");
}