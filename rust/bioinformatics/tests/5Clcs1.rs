use bioinformatics::compare::{lcs};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_lcs(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5C/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5C/outputs/".to_string() + input_file);
    let result = lcs(&input_params[0], &input_params[1]);
    println!("result ------>>>>>> {:?}", result);
    assert_eq!(result, output_params[0]);
}


#[test]
#[ignore]
fn test_lcs_1() {
    test_lcs("testset.txt");
}

#[test]
#[ignore]
fn test_lcs_2() {
    test_lcs("testset2.txt");
}

#[test]
fn test_lcs_3() {
    test_lcs("rosalind.txt");
}

#[test]
#[ignore]
fn test_lcs_4() {
    test_lcs("cogniterra.txt");
}