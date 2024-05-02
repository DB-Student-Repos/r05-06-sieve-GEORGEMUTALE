pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = vec![true; (upper_bound + 1) as usize];
    let mut result = Vec::new();
    
    for p in 2..=upper_bound {
        if primes[p as usize] {
            result.push(p);
            let mut multiple = p * p;
            while multiple <= upper_bound {
                primes[multiple as usize] = false;
                multiple += p;
            }
        }
    }
    
    result
}
