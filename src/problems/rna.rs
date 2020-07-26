use crate::mol_bio::na;
use crate::mol_bio::Chain;

pub fn solve(dna: &str) {
    let mut chain:  Chain::<na::dna::Nucleotide> = Chain::new();
    chain.from_str(dna);
    let rna = chain.to_rna();

    println!("{}", rna.to_string());
}

    
