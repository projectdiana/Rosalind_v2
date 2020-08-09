use crate::mol_bio::Chain;
use crate::mol_bio::na::dna::Nucleotide;
use std::collections::HashMap;

pub fn solve() {
    fn hashfn(x: String) -> HashMap<String, String> {
        let mut labels = HashMap::new();
        labels.insert(String::from("name"), x);
        labels
    }
    let list = Chain::from_file("/Users/jose/Programming/Rust/Rosalind_v2/src/files/hamm.txt",hashfn,Nucleotide::from_u8).unwrap();
    
    let mut max_gc: Option<(usize, f64)> = None;

    for (idx, dna) in list.iter().enumerate() {
        let total: usize = dna.loc_map.iter().map(|(key,val) | val.len()).sum();
        let gc: usize = dna.loc_map.iter().filter(|(key,val) | key == &&Nucleotide::G || key == &&Nucleotide::C).map(|(key,val) | val.len()).sum();
        let gc_content: f64 = gc as f64 / total as f64;

        if let Some(val) = max_gc {
            if gc_content > val.1 {
                max_gc = Some((idx, gc_content));
            }
        } else {
            max_gc = Some((idx, gc_content));
        }

        // println!("{:?}",
    }

    println!(">{}", list[max_gc.unwrap().0].labels.get("name").unwrap());
    println!("{}", 100.0 * max_gc.unwrap().1);

}
