use crate::common::RNA_CODON_TABLE;
use crate::common::PEPTIDE_MASS_TABLE;
use crate::common::CONDON_MASSES;
use crate::reverse_complement;
use std::collections::HashSet;
use std::collections::HashMap;

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

pub fn linear_spectrum(peptide: &String) -> Vec<usize> {
    let chars: Vec<char> = peptide.chars().collect();
    let mut prefix_mass: Vec<usize> = vec![0; chars.len() + 1];
    for i in 0..chars.len() {
        let mass = PEPTIDE_MASS_TABLE.get(&chars[i]).copied().unwrap_or(0);
        prefix_mass[i + 1] = prefix_mass[i] + mass;
    }

    let mut spectrum: Vec<usize> = vec![0];
    for i in 0..chars.len() {
        for j in (i + 1)..=chars.len() {
            spectrum.push(prefix_mass[j] - prefix_mass[i]);
        }
    }
    spectrum.sort();
    spectrum
}

pub fn linear_spectrum_masses(peptide: &Vec<usize>) -> Vec<usize> {
    let mut prefix_mass: Vec<usize> = vec![0; peptide.len() + 1];
    for i in 0..peptide.len() {
        prefix_mass[i + 1] = prefix_mass[i] + peptide[i];
    }

    let mut spectrum: Vec<usize> = vec![0];
    for i in 0..peptide.len() {
        for j in (i + 1)..=peptide.len() {
            spectrum.push(prefix_mass[j] - prefix_mass[i]);
        }
    }
    spectrum.sort();
    spectrum
}

pub fn score(peptide:&String, spectrum: Vec<usize>) -> usize {
    let peptide_spectrum = theoretical_spectrum(peptide);
    println!("peptide_spectrum = {:?}", peptide_spectrum);
    let mut correct_entries = 0;
    let mut j = 0;
    for (i, elem) in peptide_spectrum.iter().enumerate() {
        while j < spectrum.len() && *elem > spectrum[j] {
            j = j + 1;
        }
        if j < spectrum.len() && *elem == spectrum[j] {
            j = j + 1;
            correct_entries += 1;
        }
         
    }
    correct_entries
}

pub fn score_masses(peptide:Vec<usize>, spectrum: Vec<usize>) -> usize {
    let peptide_spectrum = cyclo_spectrum(&peptide);
    //println!("peptide_spectrum = {:?}", peptide_spectrum);
    let mut correct_entries = 0;
    let mut j = 0;
    for (i, elem) in peptide_spectrum.iter().enumerate() {
        while j < spectrum.len() && *elem > spectrum[j] {
            j = j + 1;
        }
        if j < spectrum.len() && *elem == spectrum[j] {
            j = j + 1;
            correct_entries += 1;
        }
         
    }
    correct_entries
}

pub fn linear_score(peptide:&String, spectrum: Vec<usize>) -> usize {
    let peptide_spectrum = linear_spectrum(peptide);
    let mut correct_entries = 0;
    let mut j = 0;
    for (i, elem) in peptide_spectrum.iter().enumerate() {
        while j < spectrum.len() && *elem > spectrum[j] {
            j = j + 1;
        }
        if j < spectrum.len() && *elem == spectrum[j] {
            j = j + 1;
            correct_entries += 1;
        }
         
    }
    correct_entries
}

pub fn linear_score_masses(peptide:&Vec<usize>, spectrum: Vec<usize>) -> usize {
    let peptide_spectrum = linear_spectrum_masses(peptide);
    let mut correct_entries = 0;
    let mut j = 0;
    for (i, elem) in peptide_spectrum.iter().enumerate() {
        while j < spectrum.len() && *elem > spectrum[j] {
            j = j + 1;
        }
        if j < spectrum.len() && *elem == spectrum[j] {
            j = j + 1;
            correct_entries += 1;
        }
         
    }
    correct_entries
}

pub fn trim(peptides: Vec<String>, spectrum:Vec<usize>, no_of_leaders: usize) -> Vec<String> {
    //println!("input params: {:?} | {:?} | {:?}", peptides, spectrum, no_of_leaders);
    let mut linear_scores:Vec<(String,usize)> = Vec::new();
    for elem in &peptides {
        linear_scores.push((elem.clone(), linear_score(&elem, spectrum.clone())));
    }

    linear_scores.sort_by(|a, b| b.1.cmp(&a.1));
    if linear_scores.is_empty() {
        return Vec::new();
    }
    if linear_scores.len() <= no_of_leaders {
        return linear_scores.into_iter().map(|x| x.0).collect();
    }

    let mut first_element_to_discard = linear_scores.len();
    for i in no_of_leaders-1..linear_scores.len() {
        if linear_scores[i].1 < linear_scores[no_of_leaders-1].1 {
            first_element_to_discard = i;
            break;
        }
    }
    linear_scores[0..first_element_to_discard].to_vec().into_iter().map(|x| x.0).collect()
}

pub fn trim_masses(peptides: HashSet<Vec<usize>>, spectrum:Vec<usize>, no_of_leaders: usize) -> HashSet<Vec<usize>> {
    //println!("input params: {:?} | {:?} | {:?}", peptides, spectrum, no_of_leaders);
    let mut linear_scores:Vec<(Vec<usize>,usize)> = Vec::new();
    for elem in &peptides {
        linear_scores.push((elem.clone(), linear_score_masses(&elem, spectrum.clone())));
    }

    linear_scores.sort_by(|a, b| b.1.cmp(&a.1));
    if linear_scores.is_empty() {
        return HashSet::new();
    }
    if linear_scores.len() <= no_of_leaders {
        return linear_scores.into_iter().map(|x| x.0).collect();
    }

    let mut first_element_to_discard = linear_scores.len();
    for i in no_of_leaders-1..linear_scores.len() {
        if linear_scores[i].1 < linear_scores[no_of_leaders-1].1 {
            first_element_to_discard = i;
            break;
        }
    }
    linear_scores[0..first_element_to_discard].to_vec().into_iter().map(|x| x.0).collect()
}

pub fn leaderboard_cyclopeptide_sequencing(spectrum: &Vec<usize>, n: usize) -> Vec<usize> {
    let parent_mass = spectrum.last().unwrap();
    let mut leader_peptide:Vec<usize> = Vec::new();
    let mut leaderboard: HashSet<Vec<usize>> = HashSet::new();
    leaderboard.insert(vec![]);
    
    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, CONDON_MASSES.to_vec());
        let mut next_candidates = leaderboard.clone();
        
        for peptide in &leaderboard {
            if mass(peptide.to_vec()) == *parent_mass {
                if score_masses(peptide.to_vec(), spectrum.to_vec()) > score_masses(leader_peptide.clone(), spectrum.to_vec()) {
                    leader_peptide = peptide.to_vec();
                }
            } else {
                if mass(peptide.to_vec()) > *parent_mass {
                    next_candidates.remove(peptide);
                }
            }
        }
        leaderboard = trim_masses(next_candidates, spectrum.to_vec(), n);
    }
    leader_peptide
}

pub fn leaderboard_cyclopeptide_sequencing_2(spectrum: &Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let parent_mass = spectrum.last().unwrap();
    let mut leader_peptides:Vec<Vec<usize>> = Vec::new();
    let mut score_leader = 0;
    let mut leaderboard: HashSet<Vec<usize>> = HashSet::new();
    leaderboard.insert(vec![]);
    
    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, CONDON_MASSES.to_vec());
        let mut next_candidates = leaderboard.clone();
        
        for peptide in &leaderboard {
            if mass(peptide.to_vec()) == *parent_mass {
                let score_peptide = score_masses(peptide.to_vec(), spectrum.to_vec());
                if score_peptide > score_leader {
                    leader_peptides = vec![peptide.to_vec()];
                    score_leader = score_masses(peptide.clone(), spectrum.to_vec());
                }
                if score_peptide == score_leader {
                    if !leader_peptides.contains(peptide) {
                        leader_peptides.push(peptide.to_vec());
                    }
                }
            } else {
                if mass(peptide.to_vec()) > *parent_mass {
                    next_candidates.remove(peptide);
                }
            }
        }
        leaderboard = trim_masses(next_candidates, spectrum.to_vec(), n);
    }
    println!("leader_peptides= {:?} {:?} {:?}", &leader_peptides, score_leader, leader_peptides.len());
    leader_peptides
}

pub fn leaderboard_cyclopeptide_sequencing_3(spectrum: &Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let parent_mass = spectrum.last().unwrap();
    let mut leader_peptides:Vec<Vec<usize>> = Vec::new();
    let mut score_leader = 0;
    let mut leaderboard: HashSet<Vec<usize>> = HashSet::new();
    leaderboard.insert(vec![]);
    
    while !leaderboard.is_empty() {
        leaderboard = expand(&leaderboard, (57..=200).collect());
        let mut next_candidates = leaderboard.clone();
        
        for peptide in &leaderboard {
            if mass(peptide.to_vec()) == *parent_mass {
                let score_peptide = score_masses(peptide.to_vec(), spectrum.to_vec());
                if score_peptide > score_leader {
                    leader_peptides = vec![peptide.to_vec()];
                    score_leader = score_masses(peptide.clone(), spectrum.to_vec());
                }
                if score_peptide == score_leader {
                    if !leader_peptides.contains(peptide) {
                        leader_peptides.push(peptide.to_vec());
                    }
                }
            } else {
                if mass(peptide.to_vec()) > *parent_mass {
                    next_candidates.remove(peptide);
                }
            }
        }
        leaderboard = trim_masses(next_candidates, spectrum.to_vec(), n);
    }
    println!("leader_peptides= {:?} {:?} {:?}", &leader_peptides, score_leader, leader_peptides.len());
    leader_peptides
}

pub fn spectral_convolution(spectrum: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut map: HashMap<usize,usize> = HashMap::new();  
    let spectrum_size = spectrum.len();
    for j in 1..spectrum_size {
        for i in 0..spectrum_size {
            println!("{:?} {:?} {:?} {:?}", i, j, &spectrum[i], &spectrum[j]);
            /*if spectrum[i] < spectrum[j] {
                let elem = spectrum[j] - spectrum[i];
                println!("{:?} {:?} {:?}", j, i, elem);
                *map.entry(elem).or_insert(0) += 1;
            }*/
        }
    }

    let mut pairs: Vec<(usize, usize)> = map.into_iter().collect();
    pairs.sort_by(|a, b| {
        b.1.cmp(&a.1) // sort by value descending
            .then_with(|| b.0.cmp(&a.0)) // tie-breaker: key descending
    });

    println!("pairs {:?}", pairs);

    let result: Vec<usize> = pairs
        .into_iter()
        .flat_map(|(key, value)| std::iter::repeat(key).take(value))
        .collect();

    result
}