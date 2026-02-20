use bioinformatics::sequencing::{linear_score};
use bioinformatics::{read_parameters_from_file_by_line, read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn test_linear_score(input_file: &str) {
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

fn trim(peptides: Vec<String>, spectrum:Vec<usize>, no_of_leaders: usize) -> Vec<String> {
    //println!("input params: {:?} | {:?} | {:?}", peptides, spectrum, no_of_leaders);
    let mut linear_scores:Vec<(String,usize)> = Vec::new();
    for elem in &peptides {
        linear_scores.push((elem.clone(), linear_score(&elem, spectrum.clone())));
    }

    linear_scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!("linear scores {:?}", linear_scores);
    let mut first_element_to_discard = 0;
    for i in no_of_leaders-1..peptides.len() {
        if linear_scores[i].1 < linear_scores[no_of_leaders-1].1 {
            println!("to compare: {:?} {:?}", &linear_scores[i], &linear_scores[no_of_leaders-1]);        
            first_element_to_discard = i;
            break;
        }
    }
    println!("first_element_to_discard {:?}", first_element_to_discard);
    linear_scores[0..first_element_to_discard].to_vec().into_iter().map(|x| x.0).collect()
}

#[test]
#[ignore]
fn test_linear_score_1() {
    test_linear_score("testset.txt");
}

#[test]
fn test_linear_score_2() {
    test_linear_score("rosalind.txt");
}

/*
#[test]
fn test_linear_score_4() {
    test_linear_score("rosalind2.txt");
} */

#[test]
#[ignore]
fn test_linear_score_3() {
    test_linear_score("cogniterra.txt");
}
