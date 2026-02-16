use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;
use rand::seq::IteratorRandom;
use rand::thread_rng;

mod common;
mod replicationorigin;
pub mod sequencing;

pub fn read_parameters_from_file(filename: String) -> Vec<String> {
    let contents = File::open(filename).expect("Failed to read file");
    let reader = BufReader::new(contents);
    let mut results = Vec::new();
    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for entry in parts.iter() {
                    results.push(entry.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    results
}

pub fn convert_parameters_to_hashmap(inputs: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_key = "".to_string();
    for entry in inputs {
        if entry.ends_with(':') {
            last_key = entry.to_string();
        } else {
            map.entry(last_key.clone()).or_insert(Vec::new()).push(entry.clone());
        }
    }
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        // Remove last character safely
        let new_key = if key.len() > 0 {
            key[..key.len() - 1].to_string()
        } else {
            key // Keep empty keys unchanged
        };
        new_map.insert(new_key, value);
    }
    new_map
}

pub fn pattern_count(text: &str, pattern: &str) -> usize {
    if pattern.is_empty() || text.len() < pattern.len() {
        return 0;
    }

    text.as_bytes()
        .windows(pattern.len())
        .filter(|window| *window == pattern.as_bytes())
        .count()
}

pub fn frequent_words(text: &str, k: usize) -> Vec<String> {
    if k == 0 || text.len() < k {
        return Vec::new();
    }

    let mut count_map = HashMap::new();

    for i in 0..=text.len() - k {
        let pattern = &text[i..i + k];
        *count_map.entry(pattern).or_insert(0) += 1;
    }

    let max_count = *count_map.values().max().unwrap_or(&0);
    
    count_map
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(pattern, _)| pattern.to_string())
        .collect()
}

pub fn reverse_complement(text: &str) -> String {
    text.chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => '-',
        })
        .collect()
}

pub fn pattern_positions(text: &str, pattern: &str) -> Vec<usize> {
    let mut results = Vec::new();
    if pattern.is_empty() {
        return results;
    }
    let mut start = 0;
    while let Some(pos) = text[start..].find(pattern) {
        let abs_pos = start + pos;
        results.push(abs_pos);
        start = abs_pos + 1;
    }
    results
}

pub fn find_clumps(text: &str, k: usize, l: usize, t: usize) -> HashSet<String> {
    let mut patterns = HashSet::new();
    let n = text.len();

    
    for i in 0..(n-l) {
        let window = &text[i..i + l];
        
        let mut freq_map = HashMap::new();

        for i in 0..=l - k {
            let pattern = &window[i..i + k];
            *freq_map.entry(pattern).or_insert(0) += 1;
        }
        
        for key in freq_map.keys() {
            if freq_map[key] >= t {
                patterns.insert(key.to_string());
            }
        }
    }
    patterns
}

pub fn minimum_skew(text: &str) -> Vec<usize> {
    let mut skew = 0;
    let mut min_skew = 0;
    let mut min_skew_indexes: Vec<usize> = vec![0];
    for (i, c) in text.chars().enumerate() {
        match c {
            'C' => skew -= 1,
            'G' => skew += 1,
            _ => skew += 0,
        }

        if skew == min_skew {
            min_skew_indexes.push(i+1);
        }

        if skew < min_skew {
            min_skew_indexes.clear();
            min_skew_indexes.push(i+1);
            min_skew = skew;
        }
    }
    min_skew_indexes
}

pub fn hamming_distance(pattern1: &str, pattern2: &str) -> usize {
    if pattern1 == pattern2 {
        return 0;
    }

    let b1 = pattern1.as_bytes();
    let b2 = pattern2.as_bytes();
    let len = b1.len().min(b2.len());

    let mut distance = 0usize;
    for i in 0..len {
        distance += (b1[i] != b2[i]) as usize;
    }

    distance + (b1.len().max(b2.len()) - len)
}

pub fn approximate_pattern_positions(text: &str, pattern: &str, distance: usize) -> Vec<usize> {
    let mut results = Vec::new();
    if pattern.is_empty() {
        return results;
    }
    let pattern_length = pattern.len();
    for i in 0..=text.len() - pattern_length {
        if hamming_distance(&text[i..i + pattern_length], &pattern) <= distance {
            results.push(i);
        }
    }
    results
}

pub fn approximate_pattern_count(text: &str, pattern: &str, distance: usize) -> usize {
    let mut result = 0;
    if pattern.is_empty() {
        return result;
    }
    let pattern_length = pattern.len();
    for i in 0..=text.len() - pattern_length {
        if hamming_distance(&text[i..i + pattern_length], &pattern) <= distance {
            result += 1;
        }
    }
    result
}

pub fn neighbors(pattern: &str, d: usize) -> Vec<String> {
    if d == 0 {
        return vec![pattern.to_string()];
    }
    if pattern.len() == 1 {
        return vec!["A".to_string(), "C".to_string(), "G".to_string(), "T".to_string()];
    }
    let mut neighborhood = HashSet::new();
    let suffix_neighbors = neighbors(pattern.get(1..).unwrap(), d);
    for text in suffix_neighbors.iter() {
        if hamming_distance(pattern.get(1..).unwrap(), text) < d {
            for c in "ACGT".chars() {
                neighborhood.insert(c.to_string() + text);
            }
        } else {
            neighborhood.insert(pattern.chars().next().unwrap().to_string() + text);
        }
    }
    neighborhood.into_iter().collect()
}

pub fn frequent_words_with_mismatches(text: &str, k: usize, d: usize) -> Vec<String> {
    let mut patterns: Vec<String> = Vec::new();
    let mut freq_map = HashMap::<String, usize>::new();
    let n = text.len();
    
    if k > n {
        return patterns;
    }
    
    for i in 0..=n - k {
        let pattern = &text[i..i + k];
        let neighborhood = neighbors(pattern, d);
        for neighbor in neighborhood {
            *freq_map.entry(neighbor).or_insert(0) += 1;
        }
    }
    
    if let Some(&max_value) = freq_map.values().max() {
        patterns = freq_map
            .into_iter()
            .filter_map(|(key, value)| {
                if value == max_value {
                    Some(key)
                } else {
                    None
                }
            })
            .collect();
    }
    patterns
}

pub fn frequent_words_with_mismatches_and_complement(text: &str, k: usize, d: usize) -> Vec<String> {
    let mut patterns: Vec<String> = Vec::new();
    let mut freq_map = HashMap::<String, usize>::new();
    let n = text.len();
    
    if k > n {
        return patterns;
    }
    
    for i in 0..=n - k {
        let pattern = &text[i..i + k];
        let neighborhood = neighbors(pattern, d);
        for neighbor in neighborhood {
            *freq_map.entry(neighbor.clone()).or_insert(0) += 1;
            *freq_map.entry(reverse_complement(&neighbor)).or_insert(0) += 1;
        }
    }
    
    if let Some(&max_value) = freq_map.values().max() {
        patterns = freq_map
            .into_iter()
            .filter_map(|(key, value)| {
                if value == max_value {
                    Some(key)
                } else {
                    None
                }
            })
            .collect();
    }
    patterns
}

pub fn motif_enumeration(dna: &[String], k: usize, d: usize) -> Vec<String> {
    let mut patterns: HashSet<String> = HashSet::new();
    let text: Vec<&str> = dna.iter().map(|s| s.as_str()).collect();
    if text.is_empty() || k == 0 {
        return Vec::new();
    }
    
    for i in 0..=text[0].len() - k {
        let pattern = &text[0][i..i + k];
        let neighborhood = neighbors(pattern, d);
        'neighbor_loop: for elem in neighborhood {
            for j in 1..text.len() {
               let pattern_cnt = approximate_pattern_count(text[j], &elem, d);
               if pattern_cnt == 0 {
                    continue 'neighbor_loop;
               }
               
            }
            patterns.insert(elem.clone());
        }
    }
    patterns.into_iter().collect()
}

pub fn score(motifs: Vec<String>) -> usize {
    if motifs.is_empty() {
        return 0;
    }
    let motifs_size = motifs.len();
    count(motifs)
        .iter()
        .map(|column| {
            let max = column.values().copied().max().unwrap_or(0);
            motifs_size - max
        })
        .sum()
}

fn my_log(x: f32) -> f32{
    if x <= 0.0 {
        return 0.0;
    }
    - x * x.log2()
} 

pub fn entropy(motifs: Vec<String>) -> f32 {
    if motifs.is_empty() {
        return f32::INFINITY;
    }
    profile(motifs)
        .iter()
        .map(|column| {
            column.values().fold(0.0, |acc, &val| acc + my_log(val))
        })
        .sum()
}

pub fn count(motifs: Vec<String>) -> Vec<HashMap<char,usize>> {
    let mut counting = Vec::new();
    let motifs_size = motifs.len();
    let k = motifs[0].len();
    for i in 0..k {
        let mut elem: HashMap<char,usize> = HashMap::new();
        for j in 0..motifs_size {
            *elem.entry(motifs[j].chars().nth(i).unwrap()).or_insert(0) += 1;
        }
        counting.push(elem);
    }
    counting
}

pub fn profile(motifs: Vec<String>) -> Vec<HashMap<char,f32>> {
    let divisor = motifs.len() as f32;
    let counts = count(motifs);
    
    
    counts.iter().map(|ele|ele.iter().map(|(k,v)| (*k, *v as f32 / divisor)).collect()).collect()
}

pub fn consensus(motifs: Vec<String>) -> String {
    if motifs.is_empty() {
        return "X".to_string();
    }
    let counts = count(motifs);

    counts
        .iter()
        .map(|column| {
            let mut best_char = 'A';
            let mut best_count = *column.get(&'A').unwrap_or(&0);
            for c in ['C', 'G', 'T'] {
                let cnt = *column.get(&c).unwrap_or(&0);
                if cnt > best_count {
                    best_char = c;
                    best_count = cnt;
                }
            }
            best_char
        })
        .collect()
}

pub fn distance_between_pattern_and_strings(pattern: &str, dna: Vec<String>) -> usize {
    let k = pattern.len();
    let mut distance = 0;
    for text in dna {
        let mut min_hamming_distance = usize::MAX;
        for i in 0..=text.len() - k {
            let pattern2 = &text[i..i+k];
            if min_hamming_distance > hamming_distance(pattern, pattern2) {
                min_hamming_distance = hamming_distance(pattern, pattern2);
            }
        }
        distance = distance + min_hamming_distance;
    }
    distance
}

pub fn all_strings(k: usize) -> Vec<String> {
    if k == 0 {
        return vec![String::new()];
    }
    let demo = "A".repeat(k);
    neighbors(demo.as_str(), k)
}

pub fn median_string(dna: Vec<String>, k: usize) -> String {
    let mut distance = usize::MAX;
    let patterns = all_strings(k);
    let mut median = "".to_string();
    for i in 0..patterns.len() {
        let pattern = patterns[i].clone();
        if distance > distance_between_pattern_and_strings(pattern.as_str(), dna.clone()) {
            distance = distance_between_pattern_and_strings(pattern.as_str(), dna.clone());
            median = pattern.clone(); 
        }
    }
    median
}

pub fn greedy_string(profile: Vec<HashMap<char,f32>>, kmer: &str) -> f32 {
    let k = kmer.len();
    let mut score: f32 = 1.0;
    for i in 0..k {
        score *= profile.get(i).unwrap().get(&kmer.chars().nth(i).unwrap()).unwrap_or(&0.0);
    }
    score
}

pub fn profile_most_probable_k_mer(text: &str, k: usize, profile:  Vec<HashMap<char,f32>>) -> String {
    let mut max_score: f32 = 0.0;
    let mut max_score_pattern = text[0..k].to_string();
    for i in 0..=text.len() - k {
        if greedy_string(profile.clone(), &text[i..i+k]) > max_score {
            max_score = greedy_string(profile.clone(), &text[i..i+k]);
            max_score_pattern = text[i..i+k].to_string();
        }
    }
    max_score_pattern
}

pub fn greedy_motif_search(k: usize, t: usize, dna: Vec<String>) -> Vec<String> {
    let mut best_motifs: Vec<String> = Vec::new();
    for i in 0..t {
        if let Some(nth_text) = dna.get(i) {
            best_motifs.push(nth_text[0..k].to_string());
        }
    }
    for i in 0..dna[0].len() - k  {
        let motif = dna[0][i..i+k].to_string();
        let mut motifs = vec![motif.clone()];
        for j in 1..t {
            let prof = profile(motifs.clone());
            let pmpk = profile_most_probable_k_mer(&dna[j],k,prof.clone());
            motifs.push(pmpk);
        }
        if score(motifs.clone()) < score(best_motifs.clone()) {
            best_motifs = motifs;
        }
    }
    best_motifs
}

pub fn count_with_pseudocount(motifs: Vec<String>) -> Vec<HashMap<char,usize>> {
    let mut counting = Vec::new();
    let motifs_size = motifs.len();
    let k = motifs[0].len();
    for i in 0..k {
        let mut elem: HashMap<char,usize> = HashMap::new();
        elem.insert('A',1);
        elem.insert('C',1);
        elem.insert('G',1);
        elem.insert('T',1);
        for j in 0..motifs_size {
            *elem.entry(motifs[j].chars().nth(i).unwrap()).or_insert(0) += 1;
        }
        counting.push(elem);
    }
    counting
}

pub fn profile_with_pseudocount(motifs: Vec<String>) -> Vec<HashMap<char,f32>> {
    let divisor = (motifs.len() + 4) as f32;
    let counts = count_with_pseudocount(motifs);
    
    
    counts.iter().map(|ele|ele.iter().map(|(k,v)| (*k, *v as f32 / divisor)).collect()).collect()
}

pub fn greedy_motif_search_with_pseudocount(k: usize, t: usize, dna: Vec<String>) -> Vec<String> {
    let mut best_motifs: Vec<String> = Vec::new();
    for i in 0..t {
        if let Some(nth_text) = dna.get(i) {
            best_motifs.push(nth_text[0..k].to_string());
        }
    }
    for i in 0..dna[0].len() - k  {
        let motif = dna[0][i..i+k].to_string();
        let mut motifs = vec![motif.clone()];
        for j in 1..t {
            let prof = profile_with_pseudocount(motifs.clone());
            let pmpk = profile_most_probable_k_mer(&dna[j],k,prof.clone());
            motifs.push(pmpk);
        }
        if score(motifs.clone()) < score(best_motifs.clone()) {
            best_motifs = motifs;
        }
    }
    best_motifs
}

pub fn motifs(profile: Vec<HashMap<char,f32>>, dna: Vec<String> ) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let k = profile.len();
    let n = dna[0].len();
    //println!("{:?}", profile.clone());
    for t in 0..dna.len() {
        let mut max_score = 0.0;
        let mut max_motif = "";
        for i in 0..=n-k {
            let motif = &dna[t][i..i+k];
            let mut value = 1.0;
            for i in 0..k {
                let mut hash:HashMap<char,f32>  = profile[i].clone();
                let c:char = motif.chars().nth(i).unwrap();
                value = value * *hash.entry(c).or_insert(0.0);
            }
            if max_score < value {
                max_score = value;
                max_motif = motif;
            }
            //println!("value, max_score {:?} {:?} {:?} {:?}", value, max_score, motif, max_motif);
        }
        result.push(max_motif.to_string());

    }
    result
}

pub fn randomized_motif_search(dna: Vec<String>, k: usize, t: usize) -> Vec<String> {
    let n = &dna[0].len();
    let no_of_motifs_in_dna = n - k + 1;
    let mut motifs_val = generate_random_motif(&dna, k);
    let mut best_motifs: Vec<String> = Vec::new();
    best_motifs = motifs_val.clone();
    //println!("{:?}", &best_motifs);
    loop {
        let prof = profile_with_pseudocount(motifs_val);
        motifs_val = motifs(prof, dna.clone());
        if score(motifs_val.clone()) < score(best_motifs.clone()) {
            best_motifs = motifs_val.clone();
        } else {
            return best_motifs;
        }
    }
    
}

pub fn random_number(n: usize) -> usize {
     rand::rng().gen_range(0..n)
}

pub fn random(p:Vec<f32>) -> usize {
     let dist = WeightedIndex::new(&p).unwrap();
     let mut rng = rand::rng();
     dist.sample(&mut rng)
}

pub fn generate_random_motif(dna: &Vec<String>, k: usize) -> Vec<String> {
    dna.iter().map(|s| {
        let i = random_number(dna[0].len()-k + 1);
        s[i..i+k].to_string()
    }).collect()
}

fn profile_probabilistic_k_mer(text: &str, k: usize, profile:  Vec<HashMap<char,f32>>) -> String {
    let mut probability_distribution = Vec::new();
    for i in 0..=text.len() - k {
        probability_distribution.push(greedy_string(profile.clone(), &text[i..i+k]));
    }
    let probable_index = random(probability_distribution);
    text[probable_index..probable_index+k].to_string()
}

pub fn gibbs_sampler(dna: &Vec<String>, k: usize, t: usize, n: usize) -> Vec<String> {
    let mut motifs = generate_random_motif(dna, k);
    let mut best_motifs = motifs.clone();
    for _ in 0..n {
        let i = random_number(t);
        let mut motifs_minus_i = motifs.clone();
        motifs_minus_i.remove(i);
        let prof = profile_with_pseudocount(motifs_minus_i.clone());
        let kmer = profile_probabilistic_k_mer(&dna[i], k, prof);
        motifs_minus_i.insert(i,kmer);
        motifs = motifs_minus_i.clone();
        if score(motifs.clone()) < score (best_motifs.clone()) {
            best_motifs = motifs_minus_i.clone();
        }

    }
    best_motifs
}

pub fn composition(strand: String, k:usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 0..=strand.len() - k {
        result.push(strand[i..i+k].to_string());
    }
    result
}


/* 
    Assumptions: 
    - all kmers in the vector have the same length
*/
/*
pub fn decomposition(kmers: Vec<String>) -> String {
    let kmer1 = &kmers[0];
    let kmer2 = &kmers[1];
    let k = kmer1.len();
    let l = kmers.len();
    let mut pos = 0;

    for i in 1..kmers.len() {
        if kmer1[i..k-i] == kmer2[0..k-i-1] {
            pos = i;
            break;
        }
    }
    println!("pos = {:?}", &pos);
    let mut result = kmer1.to_string();
    for i in 1..l {
        result.push_str(&kmers[i][k-pos..k]);
    }
    result
}*/

pub fn decomposition(kmers: Vec<String>) -> String {
    let mut result = kmers[0].to_string();
    for i in 1..kmers.len() {
        if let Some(last_char) = kmers[i].chars().last() {
            result.push(last_char);
        }
    }
    result
}

pub fn overlap_graph(kmers: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graf: HashMap<String, Vec<String>> = HashMap::new();
    let k: usize = kmers[0].len();
    for s in &kmers {
        let mut overlaps: Vec<String> = Vec::new();
        for t in &kmers {
            if s[1..k] == t[0..k-1] {
                overlaps.push(t.to_string());
            }
        }
        if overlaps.len() > 0 {
            graf.insert(s.to_string(),overlaps);
        }
    }
    //println!("Graph: {:?}", graf);
    graf
}

pub fn debruijn(strand: String, k: usize) -> HashMap<String, Vec<String>> {
    let mut debruijn_k = HashMap::new();
    for i in 0..=strand.len()-k {
        let kmer = &strand[i..i+k-1];
        if debruijn_k.contains_key(kmer) {
            continue;
        }
        for j in 0..=strand.len()-k {
            if kmer == &strand[j..j+k-1] {
                debruijn_k.entry(kmer.to_string()).or_insert(Vec::new()).push(strand[j+1..j+k].to_string());
            }
        }
    }
    debruijn_k
}

pub fn de_bruijn(kmers: Vec<String>) ->  HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let k = &kmers[0].len();
    for kmer in kmers {
        let key = &kmer[0..k-1];
        map.entry(key.to_string()).or_insert(Vec::new()).push(kmer[1..].to_string());
    }
    map
}

pub fn paired_de_bruijn(kmers: Vec<String>) ->  HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for kmer in kmers {
        let gapped_str:Vec<&str> = kmer.split('|').collect();
        let mut iterator = gapped_str.iter();
        let first_part = iterator.next().unwrap().to_string();
        let second_part = iterator.next().unwrap().to_string();
        let k = first_part.len();
        let key = format!("{}|{}", first_part[0..k-1].to_string(), second_part[0..k-1].to_string());
        let value = format!("{}|{}", first_part[1..k].to_string(), second_part[1..k].to_string());
        map.entry(key.to_string()).or_insert(Vec::new()).push(value.to_string());
    }
    map
}

pub fn eulerian_cycle(mut map: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut path: Vec<String>= Vec::new();
    let mut keys_with_exit:HashSet<String> = HashSet::new();
    let mut keys: Vec<String> = Vec::new();
    let mut no_of_keys;
    let mut start_key: String; 
    loop {
        keys = map.keys().cloned().collect();
        no_of_keys = map.len();
        if path.is_empty() {
            start_key = keys[random_number(no_of_keys)].to_string();
            path.push(start_key.clone());
        } else {
            start_key = keys_with_exit.iter().choose(&mut thread_rng()).unwrap().clone();
            let pos = path.iter().position(|r| *r == start_key).unwrap();
            path.rotate_left(pos);
            path.push(start_key.clone());
            //println!("start_key, path: {:?} {:?}", &start_key, &path);
        }

        loop {
            if let Some(next_nodes) = map.get_mut(&start_key) {    
                if next_nodes.is_empty() {
                    break;
                }
                let node_length = next_nodes.len();
                let idx = random_number(node_length);
                let new_start_key = next_nodes.remove(idx); // removes and returns owned String
                if node_length == 1 {
                    map.remove(&start_key);
                    keys_with_exit.remove(&start_key);
                } else {
                    keys_with_exit.insert(start_key.clone());
                }
                path.push(new_start_key.clone());
                start_key = new_start_key;
            } else {
                path.pop();
                //println!("remove last element! path afterwards: {:?}", path);
                break;
            }
            
            //println!("path map {:?} {:?} ", &path, &map);
            if map.is_empty() { 
                //path.pop();
                break;
            }
        }
        if map.is_empty() {
                //path.insert(0,path.last().unwrap().to_string());
                break;
        }
    }
    path
}

fn find_first_and_last_node(map: &HashMap<String, Vec<String>>) -> (String, String) {
    let mut counter: HashMap<String, usize> =  HashMap::new();
    for v in map.values() {
        for elem in v {
            *counter.entry(elem.to_string()).or_insert(0) += 1;
        }
    }
    //println!("counter: {:?}", counter);
    let mut first_node = "ERROR";
    let mut last_node = "ERROR";
    for entry in counter.keys() {
        let in_v:usize = *counter.get(entry).unwrap();
        let empty: Vec<String> = Vec::new();
        let e = map.get(entry).unwrap_or(&empty); 
        let out_v = e.len();
        if out_v > in_v {
            first_node = entry;
        }
        if out_v < in_v {
            last_node = entry;
        }
        if out_v != in_v {
            //println!("Special cases: {:?} {:?} {:?}", out_v, in_v, entry);
        }
    }
    if first_node == "ERROR" {
        for elem in map.keys() {
            if !counter.contains_key(elem) {
                first_node = elem;
            }
        }
    }
    //println!("first_node: {:?} ", first_node);
    //println!("last_node: {:?} ", last_node);
    (first_node.to_string(), last_node.to_string())
}

pub fn eulerian_path(mut map: HashMap<String, Vec<String>>) -> Vec<String> {
    let (first_node, last_node) = find_first_and_last_node(&map);
    if first_node != "ERROR".to_string() {
        map.entry(last_node.clone()).or_insert(Vec::new()).push(first_node.clone());
    }
    let mut cycle = eulerian_cycle(map);
    let mut path: Vec<String>;
    if first_node != "ERROR".to_string() { 
        cycle.pop().unwrap();
        path = cycle;
        let mut pos = 0;
        while *path.last().unwrap() != last_node {
            for (index, elem) in path.iter().enumerate() {
                if *elem == first_node && index != 0 {
                    pos = index;
                    break;
                }
            }
            path.rotate_left(pos);
            //println!("path: {:?}", path);
        }
    } else {
        path = cycle;
    }
    path
}

pub fn string_reconstruction(_k: usize, patterns: Vec<String>) -> String {
    let db = de_bruijn(patterns);
    let path = eulerian_path(db);
    decomposition(path)
}

fn glue_patterns(patterns: &Vec<String>) -> String {
    let mut d = String::new();
    for elem in patterns {
        if let Some(ch) = elem.chars().next() {
            d.push(ch);
        }
    }
    d
}

pub fn k_universal_circular(k: usize) -> String {
    let mut patterns: Vec<String> = Vec::new();
    for i in 0..2<<(k-1) {
        patterns.push(format!("{i:0width$b}", width=k));
    }
    let db = de_bruijn(patterns);
    let mut path = eulerian_cycle(db);
    path.pop();
    glue_patterns(&path)
}

pub fn string_spelled_by_patterns(patterns: &Vec<String>) -> String {
    let n = patterns.len();
    let k = patterns[0].len();
    let mut s: String = patterns[0].clone();
    if n > k {
        for i in k..n-1 {
            //println!("Add middle entry {}", i);
            if let Some(ch) = patterns[i].chars().next() {
                s.push(ch);
            }
        }
    }
    let mut part2: String = patterns.last().unwrap().to_string();
    let part1_length = s.len();
    let part2_length = part2.len();
    let requiredLen = k + n - 1;
    if part1_length + part2_length > requiredLen {
        part2 = part2[k-n+1..].to_string();
    }
    format!("{}{}",s,part2)
}

pub fn string_spelled_by_gapped_patterns(k: usize, d: usize, patterns: &Vec<String>) -> String {
    let n = patterns.len();
    let mut initials: Vec<String> = Vec::new();
    let mut terminals: Vec<String> = Vec::new();
    for elem in patterns {
        let gapped_str:Vec<&str> = elem.split('|').collect();
        let mut iterator = gapped_str.iter();
        initials.push(iterator.next().unwrap().to_string());
        terminals.push(iterator.next().unwrap().to_string());
    }
    let prefix_string = string_spelled_by_patterns(&initials);
    let suffix_string = string_spelled_by_patterns(&terminals);
    //println!("{:?} {:?}", &prefix_string,&suffix_string);
    let ls = suffix_string.len();
    for i in k + d..ls {
        let i1: usize = i;
        let j:usize = i-k-d;
        if prefix_string.get(i1..i1+1).unwrap() != suffix_string.get(j..j+1).unwrap() {
            return "there is no string spelled by the gapped patterns".to_string();
        }
    }
    format!("{}{}",prefix_string, suffix_string[ls-k-d..ls].to_string())    
}

pub fn string_from_paired_composition(k: usize, d: usize, patterns: &Vec<String>) -> String {
    //println!("patterns: {:?}", patterns);
    let db = paired_de_bruijn(patterns.clone());
    let path: Vec<String> = eulerian_path(db);
    string_spelled_by_gapped_patterns(k, d, &path)
}

pub fn maximal_non_branching_path(graph: HashMap<String,Vec<String>>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut counter: HashMap<String, usize> =  HashMap::new();
    for v in graph.values() {
        for elem in v {
            *counter.entry(elem.to_string()).or_insert(0) += 1;
        }
    }
    let nodes: Vec<&String> = graph.keys().collect();
    let mut untreated_nodes: HashSet<String> = nodes.iter().map(|s| s.to_string()).collect();
    //println!("nodes: {:?}", &nodes);
    for v in nodes {  
        let in_v:usize = *counter.entry(v.to_string()).or_insert(0);
        let empty: Vec<String> = Vec::new();
        let e = graph.get(v).unwrap_or(&empty); 
        let out_v = e.len();
        println!("if condition: {:?}, {:?}, {:?} {:?}", &v, in_v, out_v, &e);
        if !(in_v == 1 && out_v == 1) && out_v > 0 {
            for w in e {
                let mut w_prime = w.clone();
                let mut path: Vec<String> = vec![v.to_string(), w.to_string()];
                untreated_nodes.remove(v);
                untreated_nodes.remove(w);
                let mut in_w:usize = *counter.get(&w_prime).unwrap();
                let mut e1 = graph.get(&w_prime).unwrap_or(&empty); 
                let mut out_w = e1.len();
                println!("print v {:?} w {:?} in_w {:?} out_w {:?}", &v, &w, in_w, out_w);
                while in_w == 1 && out_w == 1 {
                    let u = e1.get(0).unwrap();
                    path.push(u.to_string());
                    untreated_nodes.remove(u);
                    w_prime = u.clone();
                    in_w = *counter.get(&w_prime).unwrap();
                    e1 = graph.get(&w_prime).unwrap_or(&empty);
                    out_w = e1.len();
                    println!("print v {:?} w' {:?} in_w {:?} out_w {:?}", &v, &w_prime, in_w, out_w);
                }
                paths.push(path);
            }
        }  
    }
    let mut nodes_handled:HashSet<String> = HashSet::new();
    for v in untreated_nodes {    
        let in_v:usize = *counter.entry(v.to_string()).or_insert(0);
        let empty: Vec<String> = Vec::new();
        let e = graph.get(&v).unwrap_or(&empty); 
        let out_v = e.len();
        println!("v {:?} nodes_handled {:?}", &v, &nodes_handled);
        if in_v == 1 && out_v == 1 && !nodes_handled.contains(&v) {
            let mut path: Vec<String> = vec![v.to_string()];
            nodes_handled.insert(v.to_string());
            println!("v2 {:?} nodes_handled {:?}", &v, &nodes_handled);
            let w_vec = graph.get(&v).unwrap();
            let mut w = w_vec[0].clone();
            let mut out_w = w_vec.len();
            let mut cycle_found = true;
            while out_w == 1 && w != *v {
               let next_vec = graph.get(&w).unwrap();
               path.push(w.to_string());
               nodes_handled.insert(w.to_string());
               w = next_vec[0].clone();
               out_w = graph.get(&w).unwrap_or(&empty).len();
               if out_w != 1 {
                cycle_found = false;
               }    
            }
            if cycle_found && w == *v {
                path.push(v.to_string());
                paths.push(path);
            }
        }
    }
    paths
}

pub fn contig_generation(patterns: Vec<String>) -> Vec<String>{
    let db = de_bruijn(patterns.clone());
    let contigs_raw = maximal_non_branching_path(db);
    contigs_raw.iter().map(|x| decomposition(x.to_vec())).collect()
}

