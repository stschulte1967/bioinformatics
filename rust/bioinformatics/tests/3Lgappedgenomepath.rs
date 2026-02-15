use std::collections::HashMap;
use bioinformatics::{string_spelled_by_patterns, string_spelled_by_gapped_patterns, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn test_string_spelled_by_gapped_patterns(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3L/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3L/outputs/".to_string() + input_file);
    let result = string_spelled_by_gapped_patterns(input_params[0].parse().unwrap(),
        input_params[1].parse().unwrap(),
        &input_params[2..].to_vec());
    //println!("result1 = {:?}", result);
    assert_eq!(output_params[0],result);
}

#[test]
#[ignore]
fn test_string_spelled_by_patterns() {
    let input=vec!("GACC".to_string(), "ACCG".to_string(), "CCGA".to_string(), 
        "CGAG".to_string(), "GAGC".to_string());
    assert_eq!(string_spelled_by_patterns(&input), "GACCGAGC".to_string());
}


#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_1() {
    test_string_spelled_by_gapped_patterns("testset.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_2() {
    test_string_spelled_by_gapped_patterns("rosalind.txt");
}

#[test]

fn test_string_spelled_by_gapped_patterns_3() {
    test_string_spelled_by_gapped_patterns("cogniterra.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_4() {
    test_string_spelled_by_gapped_patterns("testset2.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_5() {
    test_string_spelled_by_gapped_patterns("testset3.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_6() {
    test_string_spelled_by_gapped_patterns("testset4.txt");
}
