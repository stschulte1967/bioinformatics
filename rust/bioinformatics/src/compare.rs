use std::cmp;

pub fn dp_change(money: usize, coins:Vec<usize>) -> usize {
    let mut min_num_of_coins:Vec<usize> = vec![0];
    for m in 1..=money {
        let mut temp = usize::MAX;
        for i in 0..coins.len() {
            let coin = coins[i];
            if m >= coin {
                if min_num_of_coins[m-coin] + 1 < temp {
                    temp = min_num_of_coins[m-coin] + 1;
                }
            }
        }
        min_num_of_coins.push(temp);
    }
    min_num_of_coins[money] 
}

pub fn manhattan_tourist(n:usize, m:usize, down:&Vec<Vec<usize>>, right:&Vec<Vec<usize>>) -> usize {
    let mut s:Vec<Vec<usize>> = vec![vec![0;m+1];n+1];
    s[0][0] = 0;
    for i in 1..=n {
        s[i][0] = s[i-1][0] + down[i-1][0]
    }
    for j in 1..=m {
        s[0][j] = s[0][j-1] + right[0][j-1]
    }
    for i in 1..=n {
        for j in 1..=m {
            s[i][j] = cmp::max(s[i-1][j] + down[i-1][j], s[i][j-1] + right[i][j-1]);
        }
    }
    s[n][m]
}

pub fn lcs_back_tracking(v: &String, w: &String) -> Vec<Vec<usize>> {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<usize>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];

    for i in 0..len_v {
        s[i][0] = 0;
    }
    for j in 0..len_w {
        s[0][j] = 0;
    }
    for i in 1..len_v {
        for j in 1..len_w {
            let mut matching:usize = 0;
            if v_chars[i-1] == w_chars[j-1] {
                matching = 1;
            }
            s[i][j] = cmp::max(cmp::max(s[i-1][j], s[i][j-1]), s[i-1][j-1] + matching);
            if s[i][j] == s[i-1][j] {
                backtracking[i][j] = 0;
            } else {
                if s[i][j] == s[i][j-1] {
                    backtracking[i][j] = 1;
                } else {
                    if s[i][j] == s[i-1][j-1] + matching {
                        backtracking[i][j] = 2;
                    }
                }
            }
        }
    }
    for j in 0..len_v {
        for i in 0..len_w {
            print!("{:6}", s[j][i]);
        }
        println!("");
    }
    backtracking
}

pub fn output_lcs(backtrack: &Vec<Vec<usize>>, v:&String, i:usize, j:usize) -> String {
    if i == 0 || j == 0 {
        return "".to_string();
    }
    if backtrack[i][j] == 0 {
        return output_lcs(backtrack, v, i-1, j);
    } else {
        if backtrack[i][j] == 1 { 
            return output_lcs(backtrack, v, i,j-1);
        } else {
            return output_lcs(backtrack, v, i-1, j-1) + &v.chars().nth(i-1).unwrap().to_string();
        }
    }
}

pub fn lcs(s1: &String, s2: &String) -> String {
    let backtracking = lcs_back_tracking(s1, s2);
    println!("Backtracking =  {:?}", backtracking);
    output_lcs(&backtracking, s1, s1.len(), s2.len())
}

pub fn topological_ordering(graph: Vec<(String, String, usize)>, start_node: String, end_node: String) -> Vec<String> {
    let mut g = graph.clone();
    let mut result: Vec<String> = Vec::new();
    let mut candidates: Vec<String> = vec![start_node];
    while candidates.len() > 0 {
        let a = candidates[0];
        candidates.remove(0);
        list.push(a);
        for elem in g.filter(|x| x.0 == a).collect() {
            g.remove(elem);;
            if in_v(g, elem.1) == 0 {
                candidates.push(elem.1)
            }          
        }
    }
}