use bioinformatics::compare::{dag_lcs};
use bioinformatics::{read_parameters_from_file};

fn convert_vec_to_edges(input:&Vec<String>) -> Vec<(String, String, usize)> {
    let mut result:Vec<(String, String, usize)> = Vec::new();
    for i in 0..input.len()/3 {
        result.push((input[3*i].clone(), input[3*i+1].clone(), input[3*i+2].parse().unwrap()));
    }
    result
} 

fn test_dag(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5D/inputs/".to_string() + input_file);
    
    let output_params = read_parameters_from_file("../../data/5D/outputs/".to_string() + input_file);
    let edges = convert_vec_to_edges(&input_params[2..].to_vec());
    let (len, result) = dag_lcs(&input_params[0], &input_params[1], edges);

    println!("result ----->>>>>> {:?}", result);
    assert_eq!(len, output_params[0].parse().unwrap());
    assert_eq!(result, output_params[1..]);
}


#[test]
#[ignore]
fn test_lcs_1() {
    test_dag("testset.txt");
}

#[test]
#[ignore]
fn test_lcs_2() {
    test_dag("testset2.txt");
}

#[test]
#[ignore]
fn test_lcs_3() {
    test_dag("testset3.txt");
}

#[test]
#[ignore]
fn test_lcs_4() {
    test_dag("testset4.txt");
}

#[test]
#[ignore]
fn test_lcs_5() {
    test_dag("testset5.txt");
}

#[test]
#[ignore]
fn test_lcs_6() {
    test_dag("testset6.txt");
}

#[test]
#[ignore]
fn test_lcs_7() {
    test_dag("testset7.txt");
}

#[test]
#[ignore]
fn test_lcs_8() {
    test_dag("cogniterra.txt");
}

#[test]

fn test_lcs_9() {
    test_dag("rosalind.txt");
}
