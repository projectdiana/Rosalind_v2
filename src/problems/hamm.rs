use crate::mol_bio::Chain;
use crate::mol_bio::na::dna::Nucleotide;

pub fn solve() {
    // let test = Chain::from_file("/Users/Josiecat/Downloads/rosalind_orf_1_dataset.txt", |x| x,  Nucleotide::from_u8);
    let mut test = Chain::<Nucleotide>::new();
    test.from_str("GGGACGAGCCTATCAAACTACCAAACATAGCTAGGAAGCCCGGCTGCGAGGTCACACGGTCAACCCGATATTGTCCTATGTTCTAACGAACTGTAACGTTGCGAAAAAGGTAGGATCTGATTGCGTGTACATTAGCGCTAGATCAGATATGGATGCTCTGTACCAAAGACTCCGTAACGCTGGGAGACCGTTCTATTAAATAGACTTCGGCCCTGCCCTCCACGCGAACGCTCCTAGACGAGGACGGGACGAGGCCGGTACCAACGCCTTGGATCCTACGCGAGAACAAAACGCATGTTGTGAGCTATAATCTCTGGCCTATAGTCGAAAGCCTTCTGGCTCCAGGCACGTCGTCGTTACTCGCATTAGATAAGGCACAGGGTTTCTAGAAGAGTTCCGTGCGGAGGCCCTGCCAAACAAACCCTAGGCCGCTACAGTGCACCCGCTCGATAATTCGAGCTAAAGCCCCTTGACTGCGGAGACTTATCGCTAAGGTGCCGTTGTCGGAACCGCCGAGGCGACGGTCGCCGTTAGTTAATCCGTGCGCTTGCTGGAATGGACCTTGGAACAAATAACGACCATGAGGTGTGGGGTCCTACACTGTGGCCTCCTGCAAACACCACACGTTTATCTTGTTGTAACAGGGTAACAATGTGACCTCCATATATGCAGGGCCAAACCACTTGATTACTCGACTCTGCGTTACCTTGTGGAACCCCGGACTTGTAGGATCCACGCTCCAGCAGTCCTAGTACGCGCCCTGGAAGTAAGAGCAGAATGGACAGCCATGATACGCTCATCTCTAATGCGAGCCCCGGACCGTGAACGCTGGCGTACTTGATACGTTTACGAGAGCAACAGTGGCGCGTTTCACCTCCAACGAACACTCCATAGCTAGGTAGAGAGTGTTAGTACCGACGAATACGCCGTGCTGT");
    let mut test2 = Chain::<Nucleotide>::new();
    test2.from_str("GGGACTTCTCCCTCAACCAAAATTACATCGCTAGCAGGCACGCCCGCAAAGGTATACGCCTCACACATTAGTGTCATATAGTCTGACCAATTGGGGCCATGCTGCGAACTTAGGACTTTATTGCTTATGCATAAGCGAGCCACATGATGACGATAATATTCTTTTGATACTCACCAACTCATCGTGAACGCTCTACGAAATCTACATCGACCCATACTTCAACACTAACGCTTAGAGATGGACACGGAGCGTGACCTATCTAAGGACGTTGCATGCCACGCCATGGCTAAGTGATATGCGTGAACCCTTAAAGTTGTCCTAGACTCAAAGGCCTTTCGAATGTAGAAATGCAGACACAAGTGGCTGCGTACAGAGCCAAGTGTATCTTTAAAAGATAAATGTCGGGCCACTGCCCTCTAGGGGCTGTAATGCAGCAGAGCTTTCGTCCTAGGATCATCGTCAGAGTCCATACGCTGGGGAAACCTCACGTATGGGGTCGTTTGTTTGAACCGACGAGGGGCCGATCCCCTTGTGTGAATTCACCATCCTTTTGGGATTGTCTCGGGAAAATTGCACCCCCCTGTTCAGCAGCATGTCACCCTGCTGCGCAGCGTTAGTGCTAGCCGAATCAGTTCGTGTTAACGCAGGCATACGATACTTGCGAATCAATCGAGCCTCACCGCTGGACTGCGGTAAACGCCGTTACCCACTAGACCCACGAACGCCTAAGTTCCATGTCAACGATGACCTTCAGAGCGGCGTTTACGTTCGGGAGTAATCGTCGGCCAACATACCGGTTACTACAATGACATACACGGCTAGGGAACACGGGCTCAGATACTAAATTTACGAATGGGGCAGAAGATAATTTCACATTCTTGGCCAACTCTATCGCATTGTTCTCTCGGATCATACAAGACAACATAACTTTATTT");

    let count = test.hamming_distance(test2);

    println!("{:?}", count);

}
