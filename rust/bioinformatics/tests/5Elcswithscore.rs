use bioinformatics::compare::{lcs_with_score};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_lcs(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5E/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5E/outputs/".to_string() + input_file);
    let (lcs, r1, r2) = lcs_with_score(input_params[0].parse().unwrap(), input_params[1].parse().unwrap(),input_params[2].parse().unwrap(), &input_params[3],&input_params[4]);
    println!("result ------>>>>>> {}\n{:?}\n{:?}", lcs, r1, r2);
    assert_eq!(lcs, output_params[0].parse().unwrap());
    assert_eq!(r1, output_params[1]);
    assert_eq!(r2, output_params[2]);
}


#[test]
fn test_lcs_1() {
    test_lcs("testset.txt");
}

#[test]
#[ignore]
fn test_lcs_2() {
    test_lcs("testset2.txt");
}

#[test]
#[ignore]
fn test_lcs_3() {
    test_lcs("rosalind.txt");
}

#[test]
#[ignore]
fn test_lcs_4() {
    test_lcs("cogniterra.txt");
}