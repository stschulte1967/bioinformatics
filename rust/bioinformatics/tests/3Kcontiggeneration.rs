use std::collections::HashMap;
use bioinformatics::{contig_generation, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn test_contig_generation(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3K/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3K/outputs/".to_string() + input_file);
    let result = contig_generation(input_params.to_vec());
    println!("result1 = {:?}", result);
    //assert_eq!(output_params[0],result);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_contig_generationn_1() {
    test_contig_generation("testset.txt");
}

#[test]
#[ignore]
fn test_contig_generationn_4() {
    test_contig_generation("testset2.txt");
}

#[test]
#[ignore]
fn test_contig_generationn_5() {
    test_contig_generation("testset3.txt");
}

#[test]
#[ignore]
fn test_contig_generationn_6() {
    test_contig_generation("testset4.txt");
}

#[test]
#[ignore]
fn test_contig_generationn_7() {
    test_contig_generation("testset5.txt");
}

#[test]
#[ignore]
fn test_contig_generationn_8() {
    test_contig_generation("testset6.txt");
}

#[test]
#[ignore]
fn test_contig_generation_2() {
    test_contig_generation("rosalind.txt");
}

#[test]
fn test_contig_generation_3() {
    test_contig_generation("cogniterra.txt");
}