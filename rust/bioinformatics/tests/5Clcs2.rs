use bioinformatics::compare::{lcs};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn convert_vec_to_nodes(input:&Vec<String>) -> Vec<(String, String, usize)> {
    let mut result:Vec<(String, String, usize)> = Vec::new();
    for i in 0..input.len()/3 {
        result.push((input[3*i].clone(), input[3*i+1].clone(), input[3*i+2].parse().unwrap()));
    }
    result
} 

fn test_lcs(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5D/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5D/outputs/".to_string() + input_file);
    let nodes = convert_vec_to_nodes(&input_params[2..].to_vec());
    //let result = lcs(&input_params[0], &input_params[1]);

    println!("result ------>>>>>> {:?}", nodes);
    assert_eq!(1, 2);
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