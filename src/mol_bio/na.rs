use std::fmt;
/*

fn fn_for_nuc(nuc) {
    match nuc {
        Nucleotide::A => (),
        Nucleotide::T => (),
        Nucleotide::G => (),
        Nucleotide::C => ()
    }
}
*/

pub mod dna {
    use std::fmt;

    #[derive(std::hash::Hash, Eq, Copy, Clone, Debug, PartialEq)]
    pub enum Nucleotide {
        A,
        C,
        G,
        T,
    }


    impl Nucleotide {
        pub fn from_u8(input: &u8) -> Option<Nucleotide> {
                match input {
                    b'A' => Some(Nucleotide::A),
                    b'T' => Some(Nucleotide::T),
                    b'G' => Some(Nucleotide::G),
                    b'C' => Some(Nucleotide::C),
                    _ => None
                }
        }

        pub fn to_u8(input: &Nucleotide) -> Option<u8>{
            match input {
                Nucleotide::A => Some(b'A'),
                Nucleotide::T => Some(b'T'),
                Nucleotide::G => Some(b'G'),
                Nucleotide::C => Some(b'C')
            }
        }



        pub fn from_rna(input: &super::rna::Nucleotide) -> Option<Nucleotide> {
            match input {
                super::rna::Nucleotide::A => Some(Nucleotide::A),
                super::rna::Nucleotide::C => Some(Nucleotide::C),
                super::rna::Nucleotide::G => Some(Nucleotide::G),
                super::rna::Nucleotide::U => Some(Nucleotide::T),
            }
        }

        pub fn to_rna(input: &Nucleotide) -> Option<super::rna::Nucleotide> {
            match input {
                Nucleotide::A => Some(super::rna::Nucleotide::A),
                Nucleotide::C => Some(super::rna::Nucleotide::C),
                Nucleotide::G => Some(super::rna::Nucleotide::G),
                Nucleotide::T => Some(super::rna::Nucleotide::U),
            }
        }
        
        pub fn complement(input: &Nucleotide) -> Option<Nucleotide> {
            match input {
                Nucleotide::A => Some(Nucleotide::T),
                Nucleotide::T => Some(Nucleotide::A),
                Nucleotide::G => Some(Nucleotide::C),
                Nucleotide::C => Some(Nucleotide::G)
            }
        }


    }

    impl fmt::Display for Nucleotide {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Nucleotide::A => write!(f, "A"),
                Nucleotide::C => write!(f, "C"),
                Nucleotide::G => write!(f, "G"),
                Nucleotide::T => write!(f, "T"),
            }
        }
    }
    #[derive( Clone, Debug)]
    pub struct DNA {
        // #[derive(Copy)]
        pub nucleotides: Vec<Nucleotide>,
        // #[derive(Copy)]
        pub label: String,
    }

}


pub mod rna {
    use std::fmt;

    #[derive(std::hash::Hash, Eq, Copy, Clone, Debug, PartialEq)]
    pub enum Nucleotide {
        A,
        C,
        G,
        U,
    }


    impl Nucleotide {
        pub fn from_u8(input: &u8) -> Option<Nucleotide> {
                match input {
                    b'A' => Some(Nucleotide::A),
                    b'U' => Some(Nucleotide::U),
                    b'G' => Some(Nucleotide::G),
                    b'C' => Some(Nucleotide::C),
                    _ => None
                }
        }

        pub fn to_u8(input: &Nucleotide) -> Option<u8>{
            match input {
                Nucleotide::A => Some(b'A'),
                Nucleotide::U => Some(b'U'),
                Nucleotide::G => Some(b'G'),
                Nucleotide::C => Some(b'C')
            }
        }

        pub fn from_dna(input: &super::dna::Nucleotide) -> Option<Nucleotide> {
            match input {
                super::dna::Nucleotide::A => Some(Nucleotide::A),
                super::dna::Nucleotide::C => Some(Nucleotide::C),
                super::dna::Nucleotide::G => Some(Nucleotide::G),
                super::dna::Nucleotide::T => Some(Nucleotide::U),
            }
        }

        pub fn to_dna(input: &Nucleotide) -> Option<super::dna::Nucleotide> {
            match input {
                Nucleotide::A => Some(super::dna::Nucleotide::A),
                Nucleotide::C => Some(super::dna::Nucleotide::C),
                Nucleotide::G => Some(super::dna::Nucleotide::G),
                Nucleotide::U => Some(super::dna::Nucleotide::T),
            }
        }

        pub fn complement(input: &Nucleotide) -> Option<Nucleotide> {
            match input {
                Nucleotide::A => Some(Nucleotide::U),
                Nucleotide::U => Some(Nucleotide::A),
                Nucleotide::G => Some(Nucleotide::C),
                Nucleotide::C => Some(Nucleotide::G)
            }
        }


    }

    impl fmt::Display for Nucleotide {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Nucleotide::A => write!(f, "A"),
                Nucleotide::C => write!(f, "C"),
                Nucleotide::G => write!(f, "G"),
                Nucleotide::U => write!(f, "U"),
            }
        }
    }
    #[derive( Clone, Debug)]
    pub struct DNA {
        // #[derive(Copy)]
        pub nucleotides: Vec<Nucleotide>,
        // #[derive(Copy)]
        pub label: String,
    }

}

