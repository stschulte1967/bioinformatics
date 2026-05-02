use bioinformatics::compare::{linear_space_alignment};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_linear_space_alignment(input_file: &str, output_file: &str) {
    let input_params = read_parameters_from_file("../../data/5L/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5L/outputs/".to_string() + output_file);
    let (lcs, s1, s2) = linear_space_alignment(input_params[0].parse().unwrap(), input_params[1].parse().unwrap(), input_params[2].parse().unwrap(), 
                                        &input_params[3], &input_params[4]);
    println!("result ------>>>>>> lcs: {} s1: {:?} s2:  {:?}", lcs, &s1, &s2);
    assert_eq!(lcs, output_params[0].parse().unwrap());
    assert_eq!(s1, output_params[1]);
    assert_eq!(s2, output_params[2]);
}


#[test]
#[ignore]
fn test_linear_space_alignment_1() {
    test_linear_space_alignment("input_1.txt", "output_1.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_2() {
    test_linear_space_alignment("input_2.txt", "output_2.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_3() {
    test_linear_space_alignment("input_3.txt", "output_3.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_4() {
    test_linear_space_alignment("input_4.txt", "output_4.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_5() {
    test_linear_space_alignment("input_5.txt", "output_5.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_6() {
    test_linear_space_alignment("input_6.txt", "output_6.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_7() {
    test_linear_space_alignment("input_7.txt", "output_7.txt");
}

#[test]

fn test_linear_space_alignment_8() {
    test_linear_space_alignment("cogniterra.txt", "cogniterra.txt");
}

#[test]
#[ignore]
fn test_linear_space_alignment_9() {
    test_linear_space_alignment("rosalind.txt", "rosalind.txt");
}

