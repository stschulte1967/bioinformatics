use bioinformatics::compare::{lcs3};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_lcs3(input_file: &str, output_file: &str) {
    let input_params = read_parameters_from_file("../../data/5M/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5M/outputs/".to_string() + output_file);
    let (lcs, s1, s2, s3) = lcs3(&input_params[0], &input_params[1], &input_params[2]);
    println!("result ------>>>>>> lcs: {} s1: {:?} s2:  {:?} s3: {:?}", lcs, &s1, &s2, &s3);
    assert_eq!(lcs, output_params[0].parse().unwrap());
    //assert_eq!(s1, output_params[1]);
    //assert_eq!(s2, output_params[2]);
    //assert_eq!(s3, output_params[3]);
}


#[test]

fn test_lcs3_1() {
    test_lcs3("input_1.txt", "output_1.txt");
}

#[test]
fn test_lcs3_2() {
    test_lcs3("input_2.txt", "output_2.txt");
}

#[test]
fn test_lcs3_3() {
    test_lcs3("input_3.txt", "output_3.txt");
}

#[test]
fn test_lcs3_4() {
    test_lcs3("input_4.txt", "output_4.txt");
}

#[test]
fn test_lcs3_5() {
    test_lcs3("input_5.txt", "output_5.txt");
}

#[test]
fn test_lcs3_6() {
    test_lcs3("input_6.txt", "output_6.txt");
}

#[test]
#[ignore]
fn test_lcs3_8() {
    test_lcs3("cogniterra.txt", "cogniterra.txt");
}

#[test]
#[ignore]
fn test_lcs3_9() {
    test_lcs3("rosalind.txt", "rosalind.txt");
}

