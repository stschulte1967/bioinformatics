use crate::common::RNA_CODON_TABLE;
use crate::common::PEPTIDE_MASS_TABLE;
use crate::common::CONDON_MASSES;
use crate::reverse_complement;
use std::collections::HashSet;

pub fn protein_translation(rna: &str) -> String {
    let codon_count = rna.len() / 3;
    let mut protein: Vec<&str> = Vec::with_capacity(codon_count);
    let mut i = 0;
    while i < rna.len() {
        let elem = &rna[i..i+3];
        if let Some(codon) = RNA_CODON_TABLE.get(elem) {
            if *codon == "*" {
                break;
            }
            protein.push(codon);
        }
        i += 3;
    }
    protein.join("")
}

fn to_rna(dna: &str) -> String {
    dna.chars().map(|c| if c == 'T' {'U'} else {c}).collect()
}

/// checks whether all elements of vector seq are in vector with.
/// Assumes that seq and with are ordered!
pub fn is_consistent(seq: Vec<usize>, with: Vec<usize>) -> bool {
    let mut current_pos_in_with = 0;
    let mut consistent = true;
    'outher: for elem in &seq {
        while current_pos_in_with < with.len() {
            if elem == &with[current_pos_in_with] {
                current_pos_in_with += 1;
                continue 'outher;
            } else {
                current_pos_in_with += 1;
                if current_pos_in_with < with.len() && elem < &with[current_pos_in_with] {
                    return false;
                }
            }
        }
        consistent = false;
        break;
    }
    consistent
}

pub fn peptide_encoding(dna: &str, amino_acid: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let amino_acid_in_rna_length = amino_acid.len()*3;
    let dna_rev = reverse_complement(dna);
    let mut i = 0;
    let dna_length = dna.len();
    let rna = to_rna(dna);
    let rna_rev = to_rna(&dna_rev);
    while i < dna_length - amino_acid_in_rna_length {
        let rna_segment = &rna[i..i+amino_acid_in_rna_length];
        let translated = protein_translation(rna_segment);
        if translated == amino_acid {
            result.push(dna[i..i+amino_acid_in_rna_length].to_string());
        }
        let rna_segment_rev = &rna_rev[i..i+amino_acid_in_rna_length];
        let translated_rev = protein_translation(rna_segment_rev);
        if translated_rev == amino_acid {
            result.push(reverse_complement(&dna_rev[i..i+amino_acid_in_rna_length].to_string()));
        }
        i += 1;
    }
    result
}

pub fn calculate_weight(amino_acid: &str) -> usize {
    amino_acid.chars().map(|x| PEPTIDE_MASS_TABLE.get(&x).copied().unwrap_or(0)).sum()
}

pub fn theoretical_spectrum(amino_acid: &str) -> Vec<usize> {
    let mut result: Vec<usize> = vec![0];
    result.push(calculate_weight(&amino_acid));
    let peptide = format!("{}{}", amino_acid, amino_acid);
    let l = amino_acid.len();
    for j in 1..l {
        for i in 0..l {
            result.push(calculate_weight(&peptide[i..i+j]));
        }
    }
    result.sort();
    result
}

pub fn mass(peptide: Vec<usize>) -> usize {
    peptide.into_iter().sum()
}

pub fn cyclo_spectrum(amino_acid: &Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = vec![0];
    result.push(mass(amino_acid.to_vec()));
    let peptide = amino_acid.repeat(2);
    let l = amino_acid.len();
    for j in 1..l {
        for i in 0..l {
            result.push(mass(peptide[i..i+j].to_vec()));
        }
    }
    result.sort();
    result
}

pub fn expand(peptides: &HashSet<Vec<usize>>, condon_masses: Vec<usize>) -> HashSet<Vec<usize>> {
    let mut new_peptides: HashSet<Vec<usize>> = HashSet::new();
    for mut peptide in peptides {
        for condon in &condon_masses {
            let mut new_elem = peptide.clone();
            new_elem.push(*condon);
            //println!("condon, expanded peptine {:?} {:?}", &condon, &new_elem);
            new_peptides.insert(new_elem);
        }
    }
    new_peptides
}

pub fn cyclopeptide_sequencing(spectrum: Vec<usize>) -> HashSet<Vec<usize>> {
    let parent_mass = spectrum.last().unwrap();
    let mut final_peptides: HashSet<Vec<usize>> = HashSet::new();
    let mut candidate_peptides: HashSet<Vec<usize>> = HashSet::new();
    let mut potential_elems:Vec<usize> = Vec::new();
    for elem in CONDON_MASSES {
        if spectrum.contains(&elem) {
            potential_elems.push(elem);
        }
    }
    candidate_peptides.insert(vec![]);
    
    while !candidate_peptides.is_empty() {
        candidate_peptides = expand(&candidate_peptides, potential_elems.clone());
        let mut next_candidates= candidate_peptides.clone();
        
        for peptide in &candidate_peptides {

            if mass(peptide.to_vec()) == *parent_mass {
                if cyclo_spectrum(&peptide) == spectrum {
                    final_peptides.insert(peptide.to_vec());
                }
                next_candidates.remove(peptide);
            } else {
                let mut sorted_peptide = peptide.clone();
                sorted_peptide.sort();
                if !is_consistent(sorted_peptide, spectrum.clone()) {
                    next_candidates.remove(peptide);
                }
            }
        }
        candidate_peptides = next_candidates;
    }
    final_peptides
}