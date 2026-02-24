use bioinformatics::sequencing::{score, linear_score};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_score(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4F/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4F/outputs/".to_string() + input_file);
    let spectrum: Vec<usize> = input_params[1..]
        .iter()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    let result = score(&input_params[0], spectrum);
    println!("{:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}

fn test_linear_score(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4F/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/4F/outputs/".to_string() + input_file);
    let spectrum: Vec<usize> = input_params[1..]
        .iter()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
    let result = linear_score(&input_params[0], spectrum);
    println!("{:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}



#[test]
#[ignore]
fn test_score_1() {
    test_score("testset.txt");
}

#[test]
#[ignore]
fn test_score_2() {
    test_score("rosalind.txt");
}

#[test]
#[ignore]
fn test_score_3() {
    test_score("cogniterra.txt");
}

#[test]
#[ignore]
fn test_linear_score_1() {
    test_linear_score("testset.txt");
}

#[test]
#[ignore]
fn test_score_4() {
    test_score("quiz.txt");
}

#[test]

fn test_linear_score_2() {
    test_linear_score("quiz2.txt");
}