use bioinformatics::sequencing::{trim};
use bioinformatics::{read_parameters_from_file_by_line, read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn test_trim(input_file: &str) {
    let input_params = read_parameters_from_file_by_line("../../data/4L/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4L/outputs/".to_string() + input_file);
    let spectrum: Vec<usize> = input_params[1]
        .iter()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    let result = trim(input_params[0].clone(), spectrum, input_params[2][0].parse().unwrap());
    println!("{:?}", result);
    assert!(have_same_elements(&result, &output_params));
}



#[test]
#[ignore]
fn test_trim_1() {
    test_trim("testset.txt");
}

#[test]
fn test_trim_2() {
    test_trim("rosalind.txt");
}

/*
#[test]
fn test_linear_score_4() {
    test_linear_score("rosalind2.txt");
} */

#[test]
#[ignore]
fn test_trim_3() {
    test_trim("cogniterra.txt");
}
