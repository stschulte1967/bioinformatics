use std::collections::HashSet;
use std::collections::HashMap;

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



