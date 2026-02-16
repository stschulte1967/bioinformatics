use crate::common::RNA_CODON_TABLE;
use crate::reverse_complement;

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