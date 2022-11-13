use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();
    println!("{}", solution(&mut cache, 100, 99));
}

fn solution(cache: &mut HashMap<(i32, i32), i32>, sum_to: i32, max_size: i32) -> i32 {
    if sum_to == 0 || max_size == 1 {
        1
    } else if sum_to < 0 {
        0
    } else if let Some(x) = cache.get(&(sum_to, max_size)) {
        *x
    } else {
        let x = (1..=max_size)
            .map(|i| solution(cache, sum_to - i, std::cmp::min(i, sum_to - i)))
            .sum();
        cache.insert((sum_to, max_size), x);
        x
    }
}