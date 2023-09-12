fn main() {
    println!("start");
    let max = 1_000_000;
    let mut vec = Vec::new();
    for n in 1..max {
        let mut sum = 1;
        let mut m = 2;

        while n/m > m {
            if n % m == 0 {
                sum += n/m + m;
            }
            m += 1;
        }

        vec.push(sum);
    }

    for n in 1..max {
        let length = amicable_chain_length(n, vec.clone());
        if length > 0 {
            println!("f({n}) = {length}");
        }
    }
}

fn amicable_chain_length(n: u64, next_vec: Vec<u64>) -> u64 {
    let mut all_seen = Vec::new();
    let start = n;
    let mut sum = 1;
    let mut next = next_vec[n as usize - 1];

    while next != start {
        if all_seen.contains(&next) { return 0; }
        if next == 1 { return 0; }
        if next >= next_vec.len() as u64 { return 0; }

        all_seen.push(next);

        sum += 1;
        next = next_vec[next as usize - 1];
    }

    return sum;
}
