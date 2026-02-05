use bioinformatics::{count,profile, score, entropy, consensus, all_strings, greedy_string, profile_with_pseudocount, motifs, random, generate_random_motif};
use std::collections::HashMap;

const DNA: [&str; 10] = ["TCGGGGGTTTTT", 
           "CCGGTGACTTAC",
           "ACGGGGATTTTC",
           "TTGGGGACTTTT",
           "AAGGGGACTTCC",
           "TTGGGGACTTCC",
           "TCGGGGATTCAT",
           "TCGGGGATTCCT",
           "TAGGGGAACTAC",
           "TCGGGTATAACC"];

const DNA2: [&str; 5] = [
    "TTACCTTAAC",
    "GATGTCTGTC",
    "CCGGCGTTAG",
    "CCCTAACGAG",
    "CGTCAGAGGT"];

const DNA3: [&str; 4] = [
    "TGACGTTC",
    "TAAGAGTT",
    "GGACGAAA",
    "CTGTTCGC"];

const MOTIFS: [&str;5] = [
    "TAAC",
    "GTCT",
    "CCGG",
    "ACTA",
    "AGGT"];


#[test]
#[ignore]
fn test1_count() {
    println!("{:?}", count(DNA.iter().map(|s| s.to_string()).collect()));
}

#[test]
#[ignore]
fn test1_profile() {
    println!("{:?}", profile(DNA.iter().map(|s| s.to_string()).collect()));
}

#[test]
#[ignore]
fn test1_score() {
    println!("{:?}", score(DNA.iter().map(|s| s.to_string()).collect()));
}

#[test]
#[ignore]
fn test1_entropy() {
    println!("{:?}", entropy(DNA.iter().map(|s| s.to_string()).collect()));
}

#[test]
#[ignore]
fn test1_consensus() {
    println!("{:?}", consensus(DNA.iter().map(|s| s.to_string()).collect()));
}

#[test]
#[ignore]
fn test1_all_strings() {
    println!("{:?}", all_strings(2));
}

#[test]
#[ignore]
fn test1_greedy_string() {
    let mut profile = Vec::new();
    let mut hashmap: HashMap<char, f32> = HashMap::new();
    hashmap.entry('A').or_insert(0.1);
    profile.push(hashmap);
    println!("Result: {:?}", greedy_string(profile, &"A"));
}

#[test]
#[ignore]
fn test2_greedy_string() {
    let mut profile = Vec::new();
    let mut hashmap: HashMap<char, f32> = HashMap::new();
    hashmap.entry('A').or_insert(0.1);
    profile.push(hashmap);
    let mut hashmap2: HashMap<char, f32> = HashMap::new();
    hashmap2.entry('C').or_insert(0.1);
    profile.push(hashmap2);
    println!("Result: {:?}", greedy_string(profile, &"AG"));
}

#[test]
#[ignore]
fn test1_motif() {
    let profile = profile_with_pseudocount(DNA2.iter().map(|s| s.to_string()).collect());
    println!("test1_profile: {:?}", profile);
}

#[test]
#[ignore]
fn test2_motif() {
    let profile = profile(MOTIFS.iter().map(|s| s.to_string()).collect());
    println!("test1_profile: {:?}", profile);
}

#[test]
fn test3_motif() {
    let profile = profile(MOTIFS.iter().map(|s| s.to_string()).collect());
    let dna = DNA2.iter().map(|s| s.to_string()).collect();
    let result = motifs(profile, dna);
    println!("test1_profile: {:?}", result);
}

#[test]
fn test1_random() {
    let mut map = HashMap::new();
    for i in 0..1000 {
        *map.entry(random(vec![0.1,0.2,0.3])).or_insert(0) += 1;
    }
    println!("{:?}", map);
}

#[test]
fn test_generate_random_motif() {
    println!("Motif: {:?}", generate_random_motif(&DNA2.iter().map(|&s| s.to_string()).collect(), 4));
}

#[test]
fn test_quiz() {
    let mut motif = vec!["TGT".to_string(), "GTT".to_string(), "GAA".to_string(), "TGT".to_string()];
    let mut prof = profile_with_pseudocount(motif);
    for _ in 0..5 {
        motif = motifs(prof, DNA3.iter().map(|&s| s.to_string()).collect());
        println!("Motif Quiz = {:?}", motif);
        prof = profile_with_pseudocount(motif);
    }

}