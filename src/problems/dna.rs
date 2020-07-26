use crate::mol_bio::na::dna::Nucleotide;
use crate::mol_bio::Chain;

pub fn solve (dna: &str) {
    let mut chain:  Chain<Nucleotide> = Chain::new();
    chain.from_str(dna);
    let (a,c,g,t);
    a = chain.loc_map.get(&Nucleotide::A).unwrap().len();
    c = chain.loc_map.get(&Nucleotide::C).unwrap().len();
    g = chain.loc_map.get(&Nucleotide::G).unwrap().len();
    t = chain.loc_map.get(&Nucleotide::T).unwrap().len();

    println!("{:?} {:?} {:?} {:?}",a,c,g,t);
    println!("{:?}", chain.get_count());
}
