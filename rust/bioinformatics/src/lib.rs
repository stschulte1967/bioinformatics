use std::collections::HashSet;

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

    let mut count_map = std::collections::HashMap::new();

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
        
        let mut freq_map = std::collections::HashMap::new();

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

