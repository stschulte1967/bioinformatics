use bioinformatics::compare::{dp_change};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_dpchange(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5A/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5A/outputs/".to_string() + input_file);
    let result = dp_change(input_params[0].parse().unwrap(),input_params[1..].into_iter().map(|elem: &String| elem.parse().unwrap()).collect());
    println!("result ------>>>>>> {:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}


#[test]
#[ignore]
fn test_dpchange_1() {
    test_dpchange("testset.txt");
}

#[test]
#[ignore]
fn test_dpchange_4() {
    test_dpchange("testset2.txt");
}

#[test]
#[ignore]
fn test_dpchange_2() {
    test_dpchange("rosalind.txt");
}

#[test]
fn test_dpchange_3() {
    test_dpchange("cogniterra.txt");
}