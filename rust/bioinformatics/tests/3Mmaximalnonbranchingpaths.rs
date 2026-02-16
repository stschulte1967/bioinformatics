use std::collections::HashMap;
use bioinformatics::{maximal_non_branching_path, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

fn test_maximal_non_branching_path(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/3M/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/3M/outputs/".to_string() + input_file);
    let graph =  convert_parameters_to_hashmap(&input_params);
    let result = maximal_non_branching_path(graph);
    println!("result1 = {:?}", result);
    //assert_eq!(output_params,result);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_maximal_non_branching_path_1() {
    test_maximal_non_branching_path("testset.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_2() {
    test_maximal_non_branching_path("rosalind.txt");
}

#[test]
fn test_string_spelled_by_gapped_patterns_3() {
    test_maximal_non_branching_path("cogniterra.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_4() {
    test_maximal_non_branching_path("testset2.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_5() {
   test_maximal_non_branching_path("testset3.txt");
}

#[test]
#[ignore]
fn test_string_spelled_by_gapped_patterns_6() {
    test_maximal_non_branching_path("testset4.txt");
}
