use bioinformatics::compare::{local_alignment};
use bioinformatics::read_parameters_from_file;

fn test_local_alignment(input_file: &str) {
    let input_params = read_parameters_from_file("../../data/5F/inputs/".to_string() + input_file);
    let output_params = read_parameters_from_file("../../data/5F/outputs/".to_string() + input_file);
    let (lcs, r1, r2) = local_alignment(&input_params[0], &input_params[1]);
    println!("result ------>>>>>>>  {}\n{:?}\n{:?}", lcs, r1, r2);
    assert_eq!(lcs, output_params[0].parse().unwrap());
    assert_eq!(r1, output_params[1]);
    assert_eq!(r2, output_params[2]);
}


#[test]
#[ignore]
fn test_local_alignment_1() {
    test_local_alignment("testset.txt");
}

#[test]
#[ignore]
fn test_local_alignment_2() {
    test_local_alignment("testset2.txt");
}

#[test]
#[ignore]
fn test_local_alignment_3() {
    test_local_alignment("testset3.txt");
}

#[test]
#[ignore]
fn test_local_alignment_4() {
    test_local_alignment("testset4.txt");
}

#[test]
#[ignore]
fn test_local_alignment_5() {
    test_local_alignment("testset5.txt");
}

#[test]
#[ignore]
fn test_local_alignment_6() {
    test_local_alignment("testset6.txt");
}

#[test]
#[ignore]
fn test_local_alignment_7() {
    test_local_alignment("testset7.txt");
}

#[test]
#[ignore]
fn test_local_alignment_8() {
    test_local_alignment("rosalind.txt");
}

#[test]
fn test_local_alignment_9() {
    test_local_alignment("cogniterra.txt");
}