use bioinformatics::sequencing::{linear_spectrum, theoretical_spectrum};
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

fn score(peptide:&String, spectrum: Vec<usize>) -> usize {
    let peptide_spectrum = theoretical_spectrum(peptide);
    println!("peptide_spectrum = {:?}", peptide_spectrum);
    let mut correct_entries = 0;
    let mut j = 0;
    for (i, elem) in peptide_spectrum.iter().enumerate() {
        while j < spectrum.len() && *elem > spectrum[j] {
            j = j + 1;
        }
        if j < spectrum.len() && *elem == spectrum[j] {
            j = j + 1;
            correct_entries += 1;
        }
         
    }
    correct_entries
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

fn test_score_3() {
    test_score("cogniterra.txt");
}
