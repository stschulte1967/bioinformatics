fn no_of_combinations(n:usize) -> usize {
    //let masses = vec![57,71,87,97,99,101,103,113,114,115,128,129,131,137,147,156,163,186];
    let masses = vec![2,3];
    let mut combinations = vec![0;n+1];
    combinations[0] = 1;
    for i in 1..=n {
        for elem in &masses {
            if i >= *elem {
                combinations[i] += combinations[i - *elem];
            }
        }
    }
    combinations[n]
}



#[test]
#[ignore]
fn test_combinations_1() {
    let result = no_of_combinations(1446);
    assert_eq!(result, 14712706211);
}

#[test]
fn test_combinations_2() {
    let result = no_of_combinations(21);
    assert_eq!(result, 14712706211);
}