use bioinformatics::compare::{edit_distance};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_edit_distance(input_file: &str, output_file:&str) {
    let input_params = read_parameters_from_file("../../data/5G/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5G/outputs/".to_string() + output_file);
    let result = edit_distance(&input_params[0], &input_params[1]);
    println!("result ------>>>>>> {:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}


#[test]
#[ignore]
fn test_edit_distance_1() {
    test_edit_distance("input_1.txt", "output_1.txt");
}

#[test]
#[ignore]
fn test_edit_distance_2() {
    test_edit_distance("input_2.txt", "output_2.txt");
}

#[test]
#[ignore]
fn test_edit_distance_3() {
    test_edit_distance("input_3.txt", "output_3.txt");
}

#[test]
#[ignore]
fn test_edit_distance_4() {
    test_edit_distance("input_4.txt", "output_4.txt");
}

#[test]
#[ignore]
fn test_edit_distance_5() {
    test_edit_distance("input_5.txt", "output_5.txt");
}

#[test]
#[ignore]
fn test_edit_distance_6() {
    test_edit_distance("rosalind.txt", "rosalind.txt");
}

#[test]

fn test_edit_distance_7() {
    test_edit_distance("cogniterra.txt", "cogniterra.txt");
}