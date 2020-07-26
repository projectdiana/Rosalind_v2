use crate::mol_bio::Chain;
use crate::mol_bio::na::dna::Nucleotide;

pub fn solve() {
    let test = Chain::from_file("/Users/Josiecat/Downloads/rosalind_orf_1_dataset.txt", |x| x,  Nucleotide::from_u8);

    println!("{:?}", test);

}
