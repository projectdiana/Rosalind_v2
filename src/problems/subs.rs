use crate::mol_bio::Chain;
use crate::mol_bio::na;


pub fn solve(source: &str, search: &str) {
    let mut have = Chain::<na::dna::Nucleotide>::new();
    have.from_str(source);
    let mut want = Chain::<na::dna::Nucleotide>::new();
    want.from_str(search);

    //account for 1 indexing
    let results: Vec<usize> = have.find_pattern(&want).iter().map(|x| x+1).collect();
    println!("{:?}",results);

    
}
