use std::collections::HashMap;
use bioinformatics::{random_number, eulerian_cycle, convert_parameters_to_hashmap, read_parameters_from_file};
use std::collections::HashSet;

#[test]
#[ignore]
fn test_read_hashmap() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    println!("map ={:?}", map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle2() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    println!("path: {:?}", cycle);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_2() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/testset3.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_3() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_1.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_4() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_2.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_5() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_3.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_6() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_4.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_7() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_5.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_8() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_6.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}

#[test]
#[ignore]
fn test_build_cycle_9() {
    let input_params = read_parameters_from_file("../../data/3F/inputs/input_7.txt".to_string());
    let map =  convert_parameters_to_hashmap(&input_params);
    let cycle = eulerian_cycle(map);
    assert_eq!(1,2);
}
