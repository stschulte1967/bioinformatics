use bioinformatics::sequencing::{leaderboard_cyclopeptide_sequencing, leaderboard_cyclopeptide_sequencing_2, leaderboard_cyclopeptide_sequencing_3};
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

fn is_cyclic_permutation(a: &[usize], b: &[usize]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if a.is_empty() {
        return true;
    }

    for start in 0..a.len() {
        let mut forward_match = true;
        for i in 0..a.len() {
            if a[(start + i) % a.len()] != b[i] {
                forward_match = false;
                break;
            }
        }
        if forward_match {
            return true;
        }

        let mut reverse_match = true;
        for i in 0..a.len() {
            let idx = (start + a.len() - (i % a.len())) % a.len();
            if a[idx] != b[i] {
                reverse_match = false;
                break;
            }
        }
        if reverse_match {
            return true;
        }
    }

    false
}

fn test_leaderboard_cyclopeptide_sequencing(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4G/inputs/".to_string() + input_file);
    let n: usize = input_params[0].parse().unwrap();
    let spectrum: Vec<usize> = input_params[1..]
        .iter()
        .map(|elem| elem.parse().unwrap())
        .collect();

    let output_params = read_parameters_from_file("../../data/4G/outputs/".to_string() + input_file);
    let expected: Vec<usize> = output_params[0]
        .split('-')
        .map(|elem| elem.parse().unwrap())
        .collect();

    let result = leaderboard_cyclopeptide_sequencing(&spectrum, n);
    assert!(
        is_cyclic_permutation(&result, &expected),
        "result {:?} is not cyclically equivalent to expected {:?}",
        result,
        expected
    );
}

fn test_leaderboard_cyclopeptide_sequencing_2(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4G/inputs/".to_string() + input_file);
    let n: usize = input_params[0].parse().unwrap();
    let spectrum: Vec<usize> = input_params[1..]
        .iter()
        .map(|elem| elem.parse().unwrap())
        .collect();

    let output_params = read_parameters_from_file("../../data/4G/outputs/".to_string() + input_file);
    let expected: Vec<usize> = output_params[0]
        .split('-')
        .map(|elem| elem.parse().unwrap())
        .collect();

    let result = leaderboard_cyclopeptide_sequencing_2(&spectrum, n);
    println!("result: {:?}", result);
    /*assert!(
        is_cyclic_permutation(&result, &expected),
        "result {:?} is not cyclically equivalent to expected {:?}",
        result,
        expected
    );*/
    assert_eq!(1,2);
}

fn test_leaderboard_cyclopeptide_sequencing_3(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/4G/inputs/".to_string() + input_file);
    let n: usize = input_params[0].parse().unwrap();
    let spectrum: Vec<usize> = input_params[1..]
        .iter()
        .map(|elem| elem.parse().unwrap())
        .collect();

    let output_params = read_parameters_from_file("../../data/4G/outputs/".to_string() + input_file);
    let expected: Vec<usize> = output_params[0]
        .split('-')
        .map(|elem| elem.parse().unwrap())
        .collect();

    let result = leaderboard_cyclopeptide_sequencing_3(&spectrum, n);
    println!("result: {:?}", result);
    /*assert!(
        is_cyclic_permutation(&result, &expected),
        "result {:?} is not cyclically equivalent to expected {:?}",
        result,
        expected
    );*/
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_1() {
    test_leaderboard_cyclopeptide_sequencing("testset.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_2a() {
    test_leaderboard_cyclopeptide_sequencing("rosalind.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_3a() {
    test_leaderboard_cyclopeptide_sequencing("cogniterra.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_4() {
    test_leaderboard_cyclopeptide_sequencing_2("testset.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_5() {
    test_leaderboard_cyclopeptide_sequencing_2("spectrum10.txt");
}

#[test]
#[ignore]
fn test_leaderboard_cyclopeptide_sequencing_6() {
    test_leaderboard_cyclopeptide_sequencing_2("spectrum25.txt");
}

#[test]

fn test_leaderboard_cyclopeptide_sequencing_7() {
    test_leaderboard_cyclopeptide_sequencing_3("spectrum10.txt");
}
