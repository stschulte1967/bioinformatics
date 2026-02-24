use bioinformatics::sequencing::{convolution_cyclopeptide_sequencing, spectral_convolution_top_m};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_parameters_from_file(filename: String) -> Vec<String> {
    let contents = File::open(filename).expect("Failed to read file");
    let reader = BufReader::new(contents);
    let mut results = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for entry in parts.iter() {
                    results.push(entry.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    results
}

fn test_convolution_cyclopeptide_sequencing(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4I/inputs/".to_string() + input_file);
    let n: usize = input_params[0].parse().unwrap();
    let spectrum: Vec<usize> = input_params[2..]
        .iter()
        .map(|elem| elem.parse().unwrap())
        .collect();

    let output_params = read_parameters_from_file("../../data/4I/outputs/".to_string() + input_file);
    let expected: Vec<usize> = output_params[0]
        .split('-')
        .map(|elem| elem.parse().unwrap())
        .collect();

    let result = convolution_cyclopeptide_sequencing(input_params[0].parse().unwrap(), input_params[1].parse().unwrap(), &spectrum);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_convolution_cyclopeptide_sequencing_1() {
    test_convolution_cyclopeptide_sequencing("testset.txt");
}

#[test]
#[ignore]
fn test_convolution_cyclopeptide_sequencing_2() {
    test_convolution_cyclopeptide_sequencing("testset2.txt");
}

#[test]
#[ignore]
fn test_convolution_cyclopeptide_sequencing_3() {
    test_convolution_cyclopeptide_sequencing("rosalind.txt");
}

#[test]

fn test_convolution_cyclopeptide_sequencing_4() {
    let nqel = vec![0,99,113,114,128,227,257,299,355,356,370,371,484];
    println!("convolution: {:?}", spectral_convolution_top_m(nqel.to_vec(), 4));
}