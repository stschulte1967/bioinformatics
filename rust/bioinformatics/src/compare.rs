use std::cmp;
use std::collections::HashMap;

pub fn dp_change(money: usize, coins:Vec<usize>) -> usize {
    let mut min_num_of_coins:Vec<usize> = Vec::with_capacity(money+1);
    for _ in 0..=money {
      min_num_of_coins.push(0);  
    }
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
        min_num_of_coins[m]=temp;
    }
    min_num_of_coins[money] 
}

pub fn dp_change_mem_optimized(money: usize, coins:Vec<usize>) -> (usize,Vec<usize>) {
    let biggest_coin = coins[0];
    let mut min_num_of_coins:Vec<usize> = Vec::with_capacity(biggest_coin);
    let mut selected_coins: Vec<usize> = Vec::new();
    for _ in 0..biggest_coin {
      min_num_of_coins.push(0);  
    }
    min_num_of_coins[0]=0;
 
    for m in 1..=money {
        let mut temp = usize::MAX;
        let mut selected_coin = usize::MAX;
        for i in 0..coins.len() {
            let coin = coins[i];
            if m >= coin {
                if min_num_of_coins[(m-coin)%biggest_coin] + 1 < temp {
                    temp = min_num_of_coins[(m-coin)%biggest_coin] + 1;
                    selected_coin = coin;
                }
            }
        }
        selected_coins.push(selected_coin);
        min_num_of_coins[m%biggest_coin]=temp;
    }
    let mut i:i64 = money as i64 -1;
    let mut result_coins:Vec<usize> = Vec::new();
    while i >= 0 {
        result_coins.push(selected_coins[i as usize]);
        i = i - selected_coins[i as usize] as i64;
    }
    (min_num_of_coins[money%biggest_coin], result_coins)  
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

fn dag_nodes(edges: Vec<(String, String, usize)>) -> HashMap<String, (usize, String)> {
    let mut nodes: HashMap<String,(usize, String)> = HashMap::new();
    let mut sorted_edges = edges.clone();
    sorted_edges.sort_by(|a,b| a.0.parse::<usize>().unwrap().cmp(&(b.0.parse::<usize>().unwrap())));
    println!("{:?}\n\n", &sorted_edges);
    for (start, end, value) in sorted_edges {
        let start_value = nodes.get(&start).map(|(score, _)| *score).unwrap_or(0);
        let entry = nodes.entry(end.clone()).or_insert((0,"".to_string()));
        if start_value + value >  entry.0 {
            *entry = (start_value + value, start);
        }
    }
    nodes
}

fn dag_backtrack(start: &String, end: &String, nodes: HashMap<String, (usize, String)>) -> (usize, Vec<String>) {
    let final_node=nodes.get(end);
    let lcd = final_node.unwrap().0;
    let mut current_node = final_node.clone();
    let mut path: Vec<String> = vec![end.to_string()];
    while current_node.unwrap().1 != *start {
        path.push(current_node.unwrap().1.clone());
        current_node = Some(nodes.get(&current_node.unwrap().1).unwrap());        
    }
    path.push(current_node.unwrap().1.clone());
    (lcd,path.into_iter().rev().collect())
}

pub fn dag_lcs(start: &String, end: &String, edges: Vec<(String, String, usize)>) -> (usize, Vec<String>) {
    let nodes:HashMap<String, (usize, String)> = dag_nodes(edges);
    println!("Fix {:?}", nodes);
    dag_backtrack(start, end, nodes)
}


pub fn lcs_with_score(reward: i64, mismatch_penalty: i64, indel_penalty: i64, s1: &String, s2: &String) -> (i64, String, String) {
    let (lcs, backtracking) = lcs_back_tracking_with_score(reward, mismatch_penalty, indel_penalty, s1, s2);
    println!("Backtracking =  {:?}", backtracking);
    output_lcs_both_strings(lcs,&backtracking, s1, s2, s1.len(), s2.len())
}

pub fn lcs_back_tracking_with_score(reward: i64, mismatch_penalty: i64, indel_penalty: i64, v: &String, w: &String) -> (i64, Vec<Vec<i64>>) {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<i64>> = vec![vec![99;len_w];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] - indel_penalty;
        backtracking[i-1][0] = 1;
    }
    for j in 1..len_w {
        s[0][j] = s[0][j-1] - indel_penalty;
        backtracking[0][j-1] = 0;
    }
    
    for i in 1..len_v {
        for j in 1..len_w {
            let matching:i64;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            } else {
                matching = - mismatch_penalty
            }
            println!("matching: {}", matching);
            s[i][j] = cmp::max(cmp::max(s[i-1][j] - indel_penalty, s[i][j-1] - indel_penalty), s[i-1][j-1] + matching);
            println!("s: {}, i: {}, j {}", s[i][j], i, j);
            if s[i][j] == s[i-1][j-1] + matching {
            //if s[i][j] == s[i-1][j] - indel_penalty{
                backtracking[i][j] = 2;
            } else {
                if s[i][j] == s[i-1][j] - indel_penalty{
                //if s[i][j] == s[i][j-1] - indel_penalty{
                    backtracking[i][j] = 0;
                } else {
                    if s[i][j] == s[i][j-1] - indel_penalty{
                    //if s[i][j] == s[i-1][j] - indel_penalty{
                    //if s[i][j] == s[i-1][j-1] + matching {
                        backtracking[i][j] = 1;
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
    (s[len_v-1][len_w-1],backtracking)
}

pub fn output_lcs_both_strings(lcs: i64, backtrack: &Vec<Vec<i64>>, s1: &String, s2:&String, i:usize, j:usize) -> (i64, String, String) {
    println!("lcs: {}, backtrack: {:?} s1: {:?} s2: {:?}",  lcs, backtrack, s1, s2);
    let s1_output = output_s1_lcs(backtrack, s1, i, j);
    let s2_output = output_s2_lcs(backtrack, s2, i, j);
    (lcs,s1_output, s2_output)
}

pub fn output_s1_lcs(backtrack: &Vec<Vec<i64>>, v:&String, i:usize, j:usize) -> String {
    let mut i1:i64 = i.clone() as i64;
    let mut j1:i64 = j.clone() as i64;
    let mut result: String = "".to_string();

    while i1 > 0 && j1 >= 0 {
        println!("1 i1: {} j1: {}", i1, j1);
        if backtrack[i1 as usize][j1 as usize] == 0 {
            result = v.chars().nth((i1-1) as usize).unwrap().to_string() + &result;
            i1 = i1 - 1;
        }
        else if backtrack[i1 as usize][j1 as usize] == 1 {
            j1 = j1 - 1;
            result = "-".to_string() + &result;
        }
        else if backtrack[i1 as usize][j1 as usize] == 2 {
            result = v.chars().nth((i1-1) as usize).unwrap().to_string() + &result;
            i1 = i1 - 1;
            j1 = j1 - 1;
        }
    }
    result
}

pub fn output_s2_lcs(backtrack: &Vec<Vec<i64>>, v:&String, i:usize, j:usize) -> String {
    let mut i1:i64 = i.clone() as i64;
    let mut j1:i64 = j.clone() as i64;
    let mut result: String = "".to_string();

    while j1 > 0 {
        println!("2 i1: {} j1: {}", i1, j1);
        if backtrack[i1 as usize][j1 as usize] == 0 {
            result = "-".to_string() + &result;
            i1 = i1 - 1;
        }
        else if backtrack[i1 as usize][j1 as usize] == 1 {
            result = v.chars().nth((j1-1) as usize).unwrap().to_string() + &result;    
            j1 = j1 - 1;
        }
        else if backtrack[i1 as usize][j1 as usize] == 2 {
            result = v.chars().nth((j1-1) as usize).unwrap().to_string() + &result;
            i1 = i1 - 1;
            j1 = j1 - 1;
        }
    }
    result
}

/* 
pub fn output_s2_lcs(backtrack: &Vec<Vec<i64>>, v:&String, i:usize, j:usize) -> String {
    if i == 0 || j == 0 {
        if i == 0 && j == 0 {
            return "".to_string();
        }
        if i == 0 {
            return v.chars().nth(j-1).unwrap().to_string();
        }
        if j == 0 {
            return "-".to_string();
        }
        return "-".to_string();
    }
    if backtrack[i][j] == 0 {
        return output_s2_lcs(backtrack, v, i-1, j) + "-";
    } else {
        if backtrack[i][j] == 1 { 
            return output_s2_lcs(backtrack, v, i,j-1) + &v.chars().nth(j-1).unwrap().to_string();
        } else {
            return output_s2_lcs(backtrack, v, i-1, j-1) + &v.chars().nth(j-1).unwrap().to_string();
        }
    }
} */