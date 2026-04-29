use std::cmp;
use std::collections::HashMap;
use crate::common::PAM25;

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
    /*for j in 0..len_v {
        for i in 0..len_w {
            //print!("{:6}", s[j][i]);
        }
        //println!("");
    }
    */
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
    //println!("Backtracking =  {:?}", backtracking);
    output_lcs(&backtracking, s1, s1.len(), s2.len())
}

fn dag_nodes(edges: Vec<(String, String, usize)>) -> HashMap<String, (usize, String)> {
    let mut nodes: HashMap<String,(usize, String)> = HashMap::new();
    let mut sorted_edges = edges.clone();
    sorted_edges.sort_by(|a,b| a.0.parse::<usize>().unwrap().cmp(&(b.0.parse::<usize>().unwrap())));
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
    dag_backtrack(start, end, nodes)
}


pub fn lcs_with_score(reward: i64, mismatch_penalty: i64, indel_penalty: i64, s1: &String, s2: &String) -> (i64, String, String) {
    let (lcs, backtracking) = lcs_back_tracking_with_score(reward, mismatch_penalty, indel_penalty, s1, s2);
    //println!("Backtracking =  {:?}", backtracking);
    output_lcs_both_strings(lcs,&backtracking, s1, s2, s1.len(), s2.len())
}

pub fn lcs_back_tracking_with_score(reward: i64, mismatch_penalty: i64, indel_penalty: i64, v: &String, w: &String) -> (i64, Vec<Vec<usize>>) {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] - indel_penalty;
        backtracking[i][0] = 0;  // Moving up from [i-1][0]
    }
    for j in 1..len_w {
        s[0][j] = s[0][j-1] - indel_penalty;
        backtracking[0][j] = 1;  // Moving left from [0][j-1]
    }
    
    for i in 1..len_v {
        for j in 1..len_w {
            let matching:i64;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            } else {
                matching = - mismatch_penalty
            }
            s[i][j] = cmp::max(cmp::max(s[i-1][j] - indel_penalty, s[i][j-1] - indel_penalty), s[i-1][j-1] + matching);
            if s[i][j] == s[i-1][j-1] + matching {
                backtracking[i][j] = 2;
            } else if s[i][j] == s[i-1][j] - indel_penalty{
                backtracking[i][j] = 0;
            } else if s[i][j] == s[i][j-1] - indel_penalty{
                backtracking[i][j] = 1;
            }
        }
    }
    /*
    for j in 0..len_v {
        for i in 0..len_w {
            //print!("{:6}", s[j][i]);
        }
        //println!("");
    }
     */
    (s[len_v-1][len_w-1],backtracking)
}

pub fn output_lcs_both_strings(lcs: i64, backtrack: &Vec<Vec<usize>>, s1: &String, s2:&String, i:usize, j:usize) -> (i64, String, String) {
    let s1_output = output_s1_lcs(backtrack, s1, i, j);
    let s2_output = output_s2_lcs(backtrack, s2, i, j);
    (lcs,s1_output, s2_output)
}

pub fn output_s1_lcs(backtrack: &Vec<Vec<usize>>, v:&String, i:usize, j:usize) -> String {
    let mut i1:i64 = i.clone() as i64;
    let mut j1:i64 = j.clone() as i64;
    let mut result: String = "".to_string();

    while i1 > 0 || j1 > 0 {
        if i1 == 0 && j1 > 0 {
            // Only j1 > 0, so we're on the first row - all left moves
            result = "-".to_string() + &result;
            j1 = j1 - 1;
        } else if j1 == 0 && i1 > 0 {
            // Only i1 > 0, so we're on the first column - all up moves
            result = v.chars().nth((i1-1) as usize).unwrap().to_string() + &result;
            i1 = i1 - 1;
        } else if backtrack[i1 as usize][j1 as usize] == 0 {
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
        } else if backtrack[i1 as usize][j1 as usize] == 3 {
            return result;
        }
    }
    result
}

pub fn output_s2_lcs(backtrack: &Vec<Vec<usize>>, v:&String, i:usize, j:usize) -> String {
    let mut i1:i64 = i.clone() as i64;
    let mut j1:i64 = j.clone() as i64;
    let mut result: String = "".to_string();

    while i1 > 0 || j1 > 0 {
        if i1 == 0 && j1 > 0 {
            // Only j1 > 0, so we're on the first row - all left moves
            result = v.chars().nth((j1-1) as usize).unwrap().to_string() + &result;
            j1 = j1 - 1;
        } else if j1 == 0 && i1 > 0 {
            // Only i1 > 0, so we're on the first column - all up moves
            result = "-".to_string() + &result;
            i1 = i1 - 1;
        } else if backtrack[i1 as usize][j1 as usize] == 0 {
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
        } else if backtrack[i1 as usize][j1 as usize] == 3 {
            return result;
        }
    }
    result
}

fn score(c1: char, c2: char) -> i64 {
    let key = format!("{}{}", c1, c2);
    PAM25.get(key.as_str()).map_or(0_i64, |v| *v as i64)
}

fn local_alignment_with_score(indel_penalty:i64, v: &String, w: &String) -> (i64, Vec<Vec<usize>>, (usize, usize, i64)) {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![i64::MIN;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = 0;
        backtracking[i][0] = 0;  // Moving up from [i-1][0]
    }
    for j in 1..len_w {
        s[0][j] = 0;
        backtracking[0][j] = 1;  // Moving left from [0][j-1]
    }
    let mut max_i = 0;
    let mut max_j = 0;
    let mut max_value = 0;

    for i in 1..len_v {
        for j in 1..len_w {
            let matching:i64 = score(v_chars[i-1],w_chars[j-1]);
            s[i][j] = cmp::max(0,cmp::max(cmp::max(s[i-1][j] - indel_penalty, s[i][j-1] - indel_penalty), s[i-1][j-1] + matching));
            if s[i][j] == s[i-1][j-1] + matching {
                backtracking[i][j] = 2;
                if s[i][j] > max_value {
                    max_i = i;
                    max_j = j;
                    max_value = s[i][j];
                }
            } else if s[i][j] == s[i-1][j] - indel_penalty{
                backtracking[i][j] = 0;
            } else if s[i][j] == s[i][j-1] - indel_penalty{
                backtracking[i][j] = 1;
            } else if s[i][j] == 0 {
                backtracking[i][j] = 3;
            } else { backtracking [i][j] = 4;}
        }
    }
    for j in 0..len_v {
        for i in 0..len_w {
            print!("({:6},{:6})", s[j][i], backtracking[j][i]);
        }
        println!("");
    }
    (s[len_v-1][len_w-1],backtracking, (max_i, max_j, max_value))
}

fn output_local_alignment(backtracking: &Vec<Vec<usize>>, s1: &String, s2: &String, i: usize, j: usize, lcs: i64) -> (i64, String, String) {
    (lcs, output_s1_lcs(&backtracking, s1, i, j), output_s2_lcs(&backtracking, s2, i, j))
}

pub fn local_alignment(s1: &String, s2: &String) -> (i64, String, String) {
    let indel_penalty=5;
    let (_lcs, backtracking, (max_i, max_j, lcs)) = local_alignment_with_score(indel_penalty, s1, s2);
    println!("{},{},{}", max_i, max_j, lcs);
    output_local_alignment(&backtracking, s1, s2, max_i, max_j, lcs)
}

pub fn edit_distance(v: &String, w: &String) -> i64 {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];
    let indel_penalty:i64 = -1;
    let mismatch_penalty:i64 = -1;
    let reward:i64 = 0;
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] + indel_penalty;
    }
    for j in 1..len_w {
        s[0][j] = s[0][j-1] + indel_penalty;
    }
    for i in 1..len_v {
        for j in 1..len_w {
            let mut matching:i64 = mismatch_penalty;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            }
            s[i][j] = cmp::max(cmp::max(s[i-1][j] + indel_penalty, s[i][j-1] + indel_penalty), s[i-1][j-1] + matching);
        }
    }
    /*for j in 0..len_v {
        for i in 0..len_w {
            print!("{:6}", s[j][i]);
        }
        println!("");
    }*/
    
    -s[len_v-1][len_w-1]
}

pub fn global_alignment_old(v: &String, w: &String, indel_penalty: i64, mismatch_penalty: i64, reward: i64) -> i64 {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] - indel_penalty;
    }
    for j in 1..len_w {
        s[0][j] = s[0][j-1] - indel_penalty;
    }
    for i in 1..len_v {
        for j in 1..len_w {
            let mut matching:i64 = - mismatch_penalty;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            }
            s[i][j] = cmp::max(cmp::max(s[i-1][j] - indel_penalty, s[i][j-1] - indel_penalty), s[i-1][j-1] + matching);
        }
    }
    println!("v: {:?} w: {:?}", v, w);
    for j in 0..len_v {
        for i in 0..len_w {
            print!("{:6}", s[j][i]);
        }
        println!("");
    }
    
    s[len_v-1][len_w-1]
}

pub fn overlap_alignment_old(v: &String, w: &String, indel_penalty: i64, mismatch_penalty: i64, reward: i64) -> (i64, String, String) {
    let mut max_global_alignment = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    for i in 0..v.len() {
        for j in 0..w.len() {
            let lcs = global_alignment_old(&v[i..v.len()].to_string(), &w[0..=j].to_string(), indel_penalty, mismatch_penalty, reward);
            if  lcs > max_global_alignment {
                max_global_alignment = lcs;
                max_i = i;
                max_j = j;
            }
        }
    }
    println!("{}{}{}", max_i, max_j, max_global_alignment);
    (max_global_alignment, "".to_string(), "".to_string())
}

pub fn global_alignment(v: &String, w: &String, indel_penalty: i64, mismatch_penalty: i64, reward: i64) -> (Vec<Vec<i64>>,Vec<Vec<usize>>) {
    let v_chars: Vec<char> = v.chars().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    let mut s:Vec<Vec<i64>> = vec![vec![98;len_w];len_v];
    let mut backtracking:Vec<Vec<usize>> = vec![vec![99;len_w];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] - indel_penalty;
        backtracking[i][0] = 0;  // Moving up from [i-1][0]
    }
    for j in 1..len_w {
        s[0][j] = s[0][j-1] - indel_penalty;
        backtracking[0][j] = 1;  // Moving up from [i-1][0]
    }
    for i in 1..len_v {
        for j in 1..len_w {
            let mut matching:i64 = - mismatch_penalty;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            }
            s[i][j] = cmp::max(cmp::max(s[i-1][j] - indel_penalty, s[i][j-1] - indel_penalty), s[i-1][j-1] + matching);
            if s[i][j] == s[i-1][j]  - indel_penalty {
                backtracking[i][j] = 0;
            } else {
                if s[i][j] == s[i][j-1] - indel_penalty {
                    backtracking[i][j] = 1;
                } else {
                    if s[i][j] == s[i-1][j-1] + matching {
                        backtracking[i][j] = 2;
                    } else {
                        backtracking[i][j] = 4;
                    }
                }
            }
        }
    }
    /*
    println!("v: {:?} w: {:?}", v, w);
    for i in 0..len_v {
        for j in 0..len_w {
            print!("{:6}", s[i][j]);
        }
        println!("");
    }
    println!("Backtracking");
    for i in 0..len_v {
        for j in 0..len_w {
            print!("{:6}", backtracking[i][j]);
        }
        println!("");
    }
    */
    (s, backtracking)
}

pub fn overlap_alignment(v: &String, w: &String, indel_penalty: i64, mismatch_penalty: i64, reward: i64) -> (i64, String, String) {
    let mut max_global_alignment = 0;
    let mut max_s:Vec<Vec<i64>> = Vec::new();
    let mut max_backtracking:Vec<Vec<usize>> = Vec::new();
    let mut max_i = 0;
    let mut max_j = 0;
    let mut lcs = 0;
    

    for i in 0..=v.len() {
        let (s,backtracking) = global_alignment(&v[i..v.len()].to_string(), w, indel_penalty, mismatch_penalty, reward);
        for j in 0..w.len() {
            if  s[v.len()-i][j+1] > max_global_alignment {
                max_global_alignment = s[v.len()-i][j+1];
                max_backtracking = backtracking.clone();
                max_i = i;
                max_j = j;
            }
        }
    }
    //println!("Final calculation:");
    let v = v[max_i..].to_string();
    let w = w[..=max_j].to_string();
    //println!("v: {:?} w: {:?}", &v, &w);
    let (s,backtracking) = global_alignment(&v, &w, indel_penalty, mismatch_penalty, reward);
    //println!("{} {}  {}", backtracking.len()-1, backtracking[0].len()-1, max_global_alignment);
    (max_global_alignment, output_s1_lcs(&backtracking, &v, v.len(), w.len()), output_s2_lcs(&backtracking, &w, v.len(), w.len()))
}

pub fn middle_edge(v: &String, w: &String, indel_penalty: i64, mismatch_penalty: i64, reward: i64) -> (usize, usize, usize, usize) {
    let v_chars: Vec<char> = v.chars().collect();
    let v_chars_rev: Vec<char> = v.chars().rev().collect();
    let w_chars: Vec<char> = w.chars().collect();
    let w_chars_rev: Vec<char> = w.chars().rev().collect();
    let len_v = v_chars.len() + 1;
    let len_w = w_chars.len() + 1;
    
    let mut s:Vec<Vec<i64>> = vec![vec![98;2];len_v];
    let mut t:Vec<Vec<i64>> = vec![vec![98;2];len_v];
    s[0][0] = 0;
    for i in 1..len_v {
        s[i][0] = s[i-1][0] - indel_penalty;
    }
    let middle = w_chars.len()/2;
    println!("middle = {}", middle);
    for j in 1..=middle+1 {
        s[0][j%2] = s[0][(j-1)%2] - indel_penalty;
        for i in 1..len_v {
            let mut matching:i64 = - mismatch_penalty;
            if v_chars[i-1] == w_chars[j-1] {
                matching = reward;
            }
            s[i][j%2] = cmp::max(cmp::max(s[i-1][j%2] - indel_penalty, s[i][(j-1)%2] - indel_penalty), s[i-1][(j-1)%2] + matching);    
        }
        println!("j: {}, s[j]: {:?}", j, &s);
    }

    t[0][0] = 0;
    for i in 1..len_v {
        t[i][0] = t[i-1][0] - indel_penalty;
    }

    for j in 1..=middle {
        t[0][j%2] = t[0][(j-1)%2] - indel_penalty;
        for i in 1..len_v {
            let mut matching:i64 = - mismatch_penalty;
            if v_chars_rev[i-1] == w_chars_rev[j-1] {
                matching = reward;
            }
            t[i][j%2] = cmp::max(cmp::max(t[i-1][j%2] - indel_penalty, t[i][(j-1)%2] - indel_penalty), t[i-1][(j-1)%2] + matching);    
        }
        println!("j: {}, t[j]: {:?}", j, &t);
    }
    
    let mut middle_row = vec![99;len_v];
    let mut middle_row_plus_1 = vec![99;len_v];

    for i in 0..len_v {
        middle_row[i] = s[i][middle%2] + t[len_v-i-1][middle%2];
        middle_row_plus_1[i] = s[i][(middle + 1)%2] + t[len_v-i-1][(middle-1)%2];
    }
    println!("mr0: {:?}, mr1: {:?}", &middle_row, &middle_row_plus_1);
    let mut max_middle_row = middle_row[0];
    let mut max_middle_row_plus_1 = middle_row_plus_1[0];
    let mut max_middle_row_pos = 0;
    let mut max_middle_row_pos_plus_1 = 0;
    let mut middle_plus_1 = middle + 1;

    for i in 1..middle_row_plus_1.len() {
        if middle_row_plus_1[i] > max_middle_row_plus_1 {
            max_middle_row_plus_1 = middle_row_plus_1[i];
            max_middle_row_pos_plus_1 = i; 
        }
    }

    for i in 1..middle_row.len() {
        if middle_row[i] > max_middle_row {
            max_middle_row = middle_row[i];
            max_middle_row_pos = i; 
        }
    }
    
    if (middle_row[(max_middle_row_pos+1)%2] > max_middle_row_plus_1) {
        middle_plus_1 = middle;
        max_middle_row_pos_plus_1 = max_middle_row_pos+1;
    }

    (max_middle_row_pos,middle,max_middle_row_pos_plus_1,middle_plus_1)
}


