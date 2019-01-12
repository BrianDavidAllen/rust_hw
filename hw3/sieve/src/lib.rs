pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut bool_vec: Vec<u64> = Vec::new();
    let mut prime_vec: Vec<u64> = Vec::new();

    for i in 0..upper_bound + 1 {
        bool_vec.push(i);
    }

    for i in 0..upper_bound {
        for z in 0..upper_bound {
            if i * z > bool_vec.len() as u64 - 1 {
                break;
            } else if z == 1 || i == 1 {
                continue;
            } else {
                bool_vec[i as usize * z as usize] = 0;
            }
        }
    }

    for i in 2..upper_bound + 1 {
        if bool_vec[i as usize] != 0 {
            prime_vec.push(i);
        }
    }

    prime_vec
}
