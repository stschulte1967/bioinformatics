use bioinformatics::sequencing::{linear_spectrum};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_linear_spectrum(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4J/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4J/outputs/".to_string() + input_file);
    let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = linear_spectrum(&input_params[0]);
    println!("{:?}", result);
    assert_eq!(&result, &output_values);
}

#[test]
#[ignore]
fn test_linear_spectrum_1() {
    let amino_acid="LEQN".to_string();
    println!("weight = {:?}", linear_spectrum(&amino_acid));
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_linear_spectrum_2() {
    let amino_acid="L".to_string();
    println!("weight = {:?}", linear_spectrum(&amino_acid));
}

#[test]
#[ignore]
fn test_linear_spectrum_3() {
    test_linear_spectrum("testset.txt");
}

#[test]
fn test_linear_spectrum_4() {
    test_linear_spectrum("rosalind.txt");
}

#[test]

fn test_linear_spectrum_5() {
    test_linear_spectrum("cogniterra.txt");
}
