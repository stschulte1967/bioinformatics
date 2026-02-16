use crate::common::RNA_CODON_TABLE;

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