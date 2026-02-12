use std::collections::HashMap;
use bioinformatics::{eulerian_path, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

#[test]
#[ignore]
fn test_build_cycle() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_2() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset2.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_3() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset3.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_4() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset4.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_5() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset5.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_6() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset6.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_7() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset7.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_8() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset8.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_path_9() {
    let input_params = read_parameters_from_file("../../data/3G/inputs/testset9.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_path(map);
    println!("path1: {:?}", cycle);
    assert_eq!(1,2);
}
