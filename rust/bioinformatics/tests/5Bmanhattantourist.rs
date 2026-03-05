use bioinformatics::compare::{manhattan_tourist};
use bioinformatics::{read_parameters_from_file, have_same_elements};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;



fn test_manhattan_tourist(input_file: &str) {
    let mut input_params = read_parameters_from_file("../../data/5B/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5B/outputs/".to_string() + input_file);
    let n:usize = input_params[0].parse().unwrap();
    let m:usize = input_params[1].parse().unwrap();
    let mut down: Vec<Vec<usize>> = Vec::new();
    let mut right: Vec<Vec<usize>> = Vec::new();
    let start = 2+n*(m+1);
    input_params.remove(start);
    let input_values:Vec<usize> = input_params.into_iter().map(|elem: String| elem.parse().unwrap()).collect();
    println!("input_values {:?}", &input_values);
    for i in 0..n {
        down.push(input_values[2+i*(m+1)..2+(i+1)*(m+1)].to_vec());
    }
    for j in 0..=n {
    right.push(input_values[start+j*(m)..start+(j+1)*(m)].to_vec());
    }

    println!("down {:?}\nright: {:?}", &down, &right);
    let result = manhattan_tourist(n,m,&down,&right);
    println!("result ------>>>>>>> {:?}", result);
    assert_eq!(result, output_params[0].parse().unwrap());
}


#[test]
#[ignore]
fn test_manhattan_tourist_1() {
    test_manhattan_tourist("testset.txt");
}

#[test]
#[ignore]
fn test_manhattan_tourist_2() {
    test_manhattan_tourist("testset2.txt");
}

#[test]
#[ignore]
fn test_manhattan_tourist_3() {
    test_manhattan_tourist("rosalind.txt");
}

#[test]
fn test_manhattan_tourist_4() {
    test_manhattan_tourist("cogniterra.txt");
}