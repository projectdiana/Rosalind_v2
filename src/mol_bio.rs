use std::collections::HashMap;
use std::fmt::Display;

pub mod na;
pub mod prot;

#[derive(Clone, Debug)]
pub struct Chain<T>
where T: std::hash::Hash + Eq + Display
{
    chain: Box<Vec<T>>,
    pub loc_map: HashMap<T,Vec<usize>>,
    pub labels: HashMap<String, String>
}

impl<T: std::hash::Hash + Eq + Display> PartialEq for Chain<T> {
    fn eq(&self, other: &Self) -> bool {
        self.chain == other.chain
    }
}



impl<T: std::hash::Hash + Eq + Clone + Copy + Display> Chain<T> {

    pub fn new() -> Chain<T> {
        let vec: Vec<T> = Vec::new();
        let map: HashMap<T, Vec<usize>> = HashMap::new();        
        let labels: HashMap<String, String> = HashMap::new();
        Chain {
            chain: Box::new(vec),
            loc_map: map,
            labels: labels
        }
    }

    fn init(&mut self) {
        let vec: Vec<T> = Vec::new();
        self.chain = Box::new(vec);
    }

    fn validate(&self, input: T) -> Option<T> {
        match &input {
            T => Some(input),
            _ => None
        }
    }

    fn add(&mut self, input: T ) {
        if let Some(link) = self.validate(input) {
            self.chain.push(link);
            
            let count = self.loc_map.entry(link).or_insert(Vec::<usize>::new());
            count.push(self.chain.len()-1);
        }
    }
    
    pub fn as_vec(&self) -> Vec<T> {
        self.to_vec(|&x| Some(x))
    }


    pub fn to_vec<U, V> (&self, func: U ) -> Vec<V>
        where U: Fn(&T) -> Option<V>
    {
        // let mut result = Vec::new();
        // for item in self.chain.iter() {
        //     if let Some(link) = func(item) {
        //         result.push(link)
        //     }
        // }
        // result
        Chain::<T>::to_vec_from_iter(self.chain.iter(), func)
    }

    fn to_vec_from_iter<'a,U: 'a,V, W> (input: impl Iterator<Item = &'a U>, func: V) -> Vec<W>
        where V: Fn(&U) -> Option<W> {
            let mut result = Vec::new();
            for item in input {
                if let Some(link) = func(item) {
                    result.push(link);
                }
            }

            result
        }

    fn to_chain<U,V>(&self, func:U) -> Chain<V>
        where U: Fn(&T) -> Option<V>,
            V: std::hash::Hash + Eq + Clone + Copy + Display
        {
            let mut chain = Chain::<V>::new();
            chain.from_iter(self.chain.iter(), func);
            chain
        }



    pub fn subsection(&self, start: usize, stop:usize)-> Chain<T> {
        let window = &self.chain[start..stop];
        let mut chain = Chain::<T>::new();
        chain.from_iter(window.iter(), |&x| Some(x));
        chain
    }


    pub fn add_iter<'a, U : 'a,V> (&mut self, input: impl Iterator<Item=&'a U>, func:V)
        where V: Fn(&U) -> Option<T>
              {
                  for item in input {
                      if let Some(link) = func(&item) {
                          self.add(link);
                      }
                  }
              }

    // pub fn add_iter3<U,V,W> (&mut self, input: U, func:V)
    //     where U: IntoIterator,
    //           U::Item: AsRef<W>,
    //           V: Fn(&W) -> Option<T>
    //           {
    //               for item in input {
    //                   if let Some(link) = func(item.as_ref()) {
    //                       self.add(link);
    //                   }
    //               }
    //           }

    pub fn from_iter<'a, U: 'a, V> (&mut self, input: impl Iterator<Item=&'a U>, func:V)
        where V: Fn(&U) -> Option<T>
              {
                  self.init();
                  self.add_iter(input, func);
              }

    pub fn get_count(&self) -> std::collections::HashMap<T, usize>
    {
        let result =  self.loc_map.iter().map(|(k, v)| (k.clone(), v.len())).collect();
        result
    }
    // returns the location of the given patten within the chain 
    pub fn find_pattern(&self, source: &Chain<T>) -> Vec<usize> {
        // get the locations containing the beginning of the chain
        if let Some(locs) = self.loc_map.get(&source.chain[0]) {
            //make another vector containing booleans for if that location 
            let mut map: Vec<bool> = locs.iter().map(|x| true).collect();
            
            // loop through every item in the chain and check if the next item is in the loc map
            for (idx,link) in source.chain.iter().enumerate().skip(1) {
                map = map.iter().enumerate().map(|(num, valid)| if *valid == true {self.loc_map.get(link)
                                                                      .unwrap_or(&vec![])
                                                                      .contains(&(locs[num]+idx))}
                                                                      else { false}).collect();
            }
            locs.iter().enumerate().filter(|(num, loc)| map[*num] == true).map(|(num, loc)| *loc).collect()
        } else {
        vec![]
        }
    }

    pub fn from_file<U, V>(filename: &str, header_func: U, str_func: V) -> std::io::Result< Vec<Chain<T>>>
        where U: Fn(String) -> HashMap<String, String>,
              V: Fn(&u8) -> Option<T> + Copy
        {
            use std::io::BufRead;
            let mut chain_list : Vec<Chain<T>> = Vec::new();
            let file = std::fs::File::open(filename)?;
            let reader = std::io::BufReader::new(file);
            let mut file_iter = reader.lines().map(|l| l.unwrap());

            let mut chain: Chain<T>;
            let mut loc: Option<usize> = None;
            for line in file_iter {

                if line.len() > 0 && line.as_bytes()[0usize]== b'>' {
                    let mut label = String::from(line);
                    label = label.replace(">","");
                    chain = Chain::new();
                    chain.labels = header_func(label);
                    chain_list.push(chain);
                    if !loc.is_some() {
                        loc = Some(0)
                    } else {
                        if let Some(i) = loc {
                            loc = Some(i+1);
                        }
                    }
                } else {
                    chain_list[loc.unwrap()].add_iter(line.as_bytes().iter(), str_func);
                }
            }

            Ok(chain_list)
        }



    pub fn hamming_distance(&self, chain: Chain<T>) -> usize {

        self.as_vec().iter().zip(chain.as_vec().iter()).filter(|x| x.0 != x.1).count()
    }
}




impl Chain<na::dna::Nucleotide> {

    pub fn from_str(&mut self, input: &str) {
        self.init();
        self.add_str(input);
    }
    
    pub fn add_str(&mut self, input: &str) {
        self.from_iter(input.as_bytes().iter(), na::dna::Nucleotide::from_u8);
    }

    pub fn to_rna(&self) -> Chain<na::rna::Nucleotide> {
        self.to_chain(na::dna::Nucleotide::to_rna)
    }

    pub fn to_string(&self) -> String {
        let vec = self.to_vec(na::dna::Nucleotide::to_u8);
        String::from_utf8(vec).unwrap()
    }

    pub fn reverse_comp(&self) -> Chain<na::dna::Nucleotide> {
        let mut chain: Chain<na::dna::Nucleotide> = Chain::new();
        // chain.from_iter(self.to_vec(
        chain.from_iter(self.chain.iter().rev(), na::dna::Nucleotide::complement);
        chain

    }



}


impl Chain<na::rna::Nucleotide> {

    pub fn from_str(&mut self, input: &str) {
        self.init();
        self.add_str(input);
    }
    
    pub fn add_str(&mut self, input: &str) {
        self.from_iter(input.as_bytes().iter(), na::rna::Nucleotide::from_u8);
    }

    pub fn to_dna(&self) -> Chain<na::dna::Nucleotide> {
        self.to_chain(na::rna::Nucleotide::to_dna)
    }

    pub fn to_string(&self) -> String {
        let vec = self.to_vec(na::rna::Nucleotide::to_u8);
        String::from_utf8(vec).unwrap()
    }

    pub fn to_codons(&self) -> Vec<prot::Codon> {
        let mut codons = vec![];
        for slice in self.chain.chunks_exact(3) {
            codons.push(prot::Codon(slice[0], slice[1], slice[2]))
        }
        codons
    }

    // pub fn to_aminos(&self) -> Vec<


    


}


