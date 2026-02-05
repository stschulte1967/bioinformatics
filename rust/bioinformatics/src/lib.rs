use std::collections::HashSet;
use std::collections::HashMap;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;

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

    let mut result = kmer1.to_string();
    for i in 1..l {
        println!("{:?}",&kmers[i][k-pos..k]);
        result.push_str(&kmers[i][k-pos..k]);
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
    println!("Graph: {:?}", graf);
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



