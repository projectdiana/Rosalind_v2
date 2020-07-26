pub fn solve(n: usize, k: usize) {
    if n < 2 {
        println!("1")
    } else {
        let mut vec :Vec<usize> = vec![1,1];
        for i in 2..n {
            vec.push(vec[i-2]*k + vec[i-1]);
        }
        println!("{}", vec.pop().unwrap());
    }
}
