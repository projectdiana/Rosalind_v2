use crate::mol_bio::na::dna::Nucleotide;
use crate::mol_bio::Chain;

pub fn solve(input: &str) {
    let mut chain: Chain<Nucleotide> = Chain::new();
    chain.from_str(input);
    let rev = chain.reverse_comp();
    println!("{}",rev.to_string());
}

