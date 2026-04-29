use bioinformatics::compare::{overlap_alignment};
use bioinformatics::{read_parameters_from_file};
use std::fs::File;



fn test_overlap_alignment(input_file: &str, output_file:&str) {
    let input_params = read_parameters_from_file("../../data/5I/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5I/outputs/".to_string() + output_file);
    let result = overlap_alignment(&input_params[3], &input_params[4], input_params[2].parse().unwrap(), input_params[1].parse().unwrap(), input_params[0].parse().unwrap());
    println!("result ------>>>>>> {:?}", result);
    assert_eq!(result.0, output_params[0].parse().unwrap());
    assert_eq!(result.1, output_params[1]);
    assert_eq!(result.2, output_params[2]);
}


#[test]
#[ignore]
fn test_overlap_alignment_1() {
    test_overlap_alignment("input_1.txt", "output_1.txt");
}

#[test]
#[ignore]
fn test_overlap_alignment_2() {
    test_overlap_alignment("input_2.txt", "output_2.txt");
}

#[test]
#[ignore]
fn test_overlap_alignment_3() {
    test_overlap_alignment("input_3.txt", "output_3.txt");
}

#[test]
#[ignore]
fn test_overlap_alignment_4() {
    test_overlap_alignment("input_4.txt", "output_4.txt");
}

#[test]
#[ignore]
fn test_overlap_alignment_5() {
    test_overlap_alignment("input_5.txt", "output_5.txt");
}

#[test]
#[ignore]
fn test_overlap_alignment_6() {
    test_overlap_alignment("rosalind.txt", "rosalind.txt");
}

#[test]

fn test_overlap_alignment_7() {
    test_overlap_alignment("cogniterra.txt", "cogniterra.txt");
}