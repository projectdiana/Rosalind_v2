mod mol_bio;
mod problems;

fn main() {
    // let mut dna: mol_bio::Chain::<mol_bio::na::dna::Nucleotide> = mol_bio::Chain::new();
    // dna.from_str("AGTCTGAAAGTTT");

    // println!("{:?}", dna);

    // let mut rna = dna.to_rna();
    // println!("{:?}", rna);

    // problems::fibd::solve(96,1,16 );
    problems::gc::solve();

}
