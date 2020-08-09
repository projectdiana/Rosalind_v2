pub fn solve(n: usize, k: u128, m: u64) {
    if n < 2 {
        println!("1")
    } else {
        let mut vec: Vec<[u128; 2]> = vec![[0,1],[1,0]];
        for i in 2..n {
            // children = adults from last round * reproduction rate
            let children: u128 = vec[i-1][0]*k;
            let adults: u128 = vec.iter()
                                   .enumerate()
                                   .filter(|(idx, val)| {
                                       (*idx < i) && (*idx as isize > i as isize - m as isize) 
                                    })
                                   .map(|x| x.1[1])
                                   .sum();
            vec.push([adults,children]);
            // println!("{:?}", vec);
            println!("{:?}, {}",vec, adults+children);

        }
    }
}
