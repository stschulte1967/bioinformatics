use bioinformatics::compare::{middle_edge, global_alignment};
use bioinformatics::{read_parameters_from_file};
use std::fs::File;



fn test_middle_edge(input_file: &str, output_file:&str) {
    let input_params = read_parameters_from_file("../../data/5K/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5K/outputs/".to_string() + output_file);
    let result = middle_edge(&input_params[4], &input_params[3], input_params[2].parse().unwrap(), input_params[1].parse().unwrap(), input_params[0].parse().unwrap());
    println!("result ------>>>>>> {:?}", result);
    assert_eq!(result.0, output_params[0].parse().unwrap());
    assert_eq!(result.1, output_params[1].parse().unwrap());
    assert_eq!(result.2, output_params[2].parse().unwrap());
    assert_eq!(result.3, output_params[3].parse().unwrap());
}

#[test]
#[ignore]
fn test_middle_edge_0() {
    test_middle_edge("ex_1.txt", "ex_1.txt");
}

#[test]

fn test_middle_edge_1() {
    test_middle_edge("input_1.txt", "output_1.txt");
}

#[test]
#[ignore]
fn test_middle_edge_2() {
    test_middle_edge("input_2.txt", "output_2.txt");
}

#[test]
#[ignore]
fn test_middle_edge_3() {
    test_middle_edge("input_3.txt", "output_3.txt");
}

#[test]
#[ignore]
fn test_middle_edge_4() {
    test_middle_edge("input_4.txt", "output_4.txt");
}

#[test]
#[ignore]
fn test_middle_edge_5() {
    test_middle_edge("input_5.txt", "output_5.txt");
}

#[test]
#[ignore]
fn test_middle_edge_6() {
    test_middle_edge("rosalind.txt", "rosalind.txt");
}

#[test]
#[ignore]
fn test_middle_edge_7() {
    test_middle_edge("cogniterra.txt", "cogniterra.txt");
}

#[test]
#[ignore]
fn test_global_alignment() {
    let v = "AT".to_string();
    let w = "GAT".to_string();
    let (s,b) = global_alignment(&v, &w, 2, 1, 1);
    println!("s: {:?} b: {:?}", &s, &b);
}