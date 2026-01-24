use bioinformatics::{count,profile, score};
use std::fs::File;
use std::io::{BufRead, BufReader};

const dna: [&str; 10] = ["TCGGGGGTTTTT", 
           "CCGGTGACTTAC",
           "ACGGGGATTTTC",
           "TTGGGGACTTTT",
           "AAGGGGACTTCC",
           "TTGGGGACTTCC",
           "TCGGGGATTCAT",
           "TCGGGGATTCCT",
           "TAGGGGAACTAC",
           "TCGGGTATAACC"];

#[test]
fn test1_count() {
    println!("{:?}", count(dna.iter().map(|s| s.to_string()).collect()));
}

#[test]
fn test1_profile() {
    println!("{:?}", profile(dna.iter().map(|s| s.to_string()).collect()));
}

#[test]
fn test1_score() {
    println!("{:?}", score(dna.iter().map(|s| s.to_string()).collect()));
}