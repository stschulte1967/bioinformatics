use bioinformatics::sequencing::{spectral_convolution};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_spectral_convolution(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4H/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4H/outputs/".to_string() + input_file);
    let output_values: Vec<usize> = output_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    let result = spectral_convolution(input_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect());
    println!("{:?}", result);
    //assert!(have_same_elements(&result, &output_values));
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_spectral_convolution_1() {
    test_spectral_convolution("testset.txt");
}

#[test]
#[ignore]
fn test_spectral_convolution_2() {
    test_spectral_convolution("rosalind.txt");
}

#[test]
#[ignore]
fn test_spectral_convolution_3() {
    test_spectral_convolution("cogniterra.txt");
}

#[test]

fn test_spectral_convolution_4() {
    test_spectral_convolution("testset2.txt");
}

#[test]
#[ignore]
fn test_spectral_convolution_5() {
    test_spectral_convolution("testset3.txt");
}
