use std::collections::HashMap;
use once_cell::sync::Lazy;

enum Nucleotide {A, C, G, T}

pub static RNA_CODON_TABLE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("AAA", "K"), ("AAC", "N"), ("AAG", "K"), ("AAU", "N"),
        ("ACA", "T"), ("ACC", "T"), ("ACG", "T"), ("ACU", "T"),
        ("AGA", "R"), ("AGC", "S"), ("AGG", "R"), ("AGU", "S"),
        ("AUA", "I"), ("AUC", "I"), ("AUG", "M"), ("AUU", "I"),
        ("CAA", "Q"), ("CAC", "H"), ("CAG", "Q"), ("CAU", "H"),
        ("CCA", "P"), ("CCC", "P"), ("CCG", "P"), ("CCU", "P"),
        ("CGA", "R"), ("CGC", "R"), ("CGG", "R"), ("CGU", "R"),
        ("CUA", "L"), ("CUC", "L"), ("CUG", "L"), ("CUU", "L"),
        ("GAA", "E"), ("GAC", "D"), ("GAG", "E"), ("GAU", "D"),
        ("GCA", "A"), ("GCC", "A"), ("GCG", "A"), ("GCU", "A"),
        ("GGA", "G"), ("GGC", "G"), ("GGG", "G"), ("GGU", "G"),
        ("GUA", "V"), ("GUC", "V"), ("GUG", "V"), ("GUU", "V"),
        ("UAA", "*"), ("UAC", "Y"), ("UAG", "*"), ("UAU", "Y"),
        ("UCA", "S"), ("UCC", "S"), ("UCG", "S"), ("UCU", "S"),
        ("UGA", "*"), ("UGC", "C"), ("UGG", "W"), ("UGU", "C"),
        ("UUA", "L"), ("UUC", "F"), ("UUG", "L"), ("UUU", "F"),
    ])
});

pub static PEPTIDE_MASS_TABLE: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    HashMap::from([
        ('G', 57), ('A', 71), ('S', 87), ('P', 97),
        ('V', 99), ('T', 101), ('C', 103), ('I', 113),
        ('L', 113), ('N', 114), ('D', 115), ('K', 128),
        ('Q', 128), ('E', 129), ('M', 131), ('H', 137),
        ('F', 147), ('R', 156), ('Y', 163), ('W', 186),
    ])
});

pub static CONDON_MASSES:[usize;18] = [57,71,87,97,99,101,103,113,114,115,128,129,131,137,147,156,163,186];