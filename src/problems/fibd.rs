pub fn solve(n: usize, k: usize, m: usize) {
    if n < 2 {
        println!("1")
    } else {
        let mut vec :Vec<usize> = vec![1,1];
        for i in 2..n {
            let mut dead: usize = 0;
            // subtract all of the children that were first alive m rounds months ago
            if i < m || i == m+1 {
                dead = 0
            } else if i ==  m {
                dead = vec[0]
            } else {
                dead = vec[i-m-1]*k;
            }
            vec.push(vec[i-2]*k + vec[i-1] - dead);
            println!("{:?}, {:?}, {}", vec, dead, i);
        }
        println!("{}", vec.pop().unwrap());
    }
}
