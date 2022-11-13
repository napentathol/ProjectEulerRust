

fn main() {
    let mut primes = vec![];

    for a in primal::Primes::all().take_while(|p| * p < 26_033) {
        let mut bidis: Vec<usize> = vec![];
        for b in &primes {
            if bi_directional_concat_prime(a, *b) { bidis.push(*b) }
        }

        /*
        if bidis.len() >= 3 {
            triple_groups(&bidis).iter()
                .filter(|x| tri_directional_concat_prime(x.0, x.1, x.2))
                .for_each(|x| {
                    println!("{} + {} + {} + {} = {}", a, x.2, x.1, x.0, a + x.2 + x.1 + x.0)
                })
        }*/

        if bidis.len() >= 4 {
            quad_groups(&bidis).iter()
                .filter(|x| quad_directional_concat_prime(x.0, x.1, x.2, x.3))
                .for_each(|x| {
                    println!("{} + {} + {} + {} + {} = {}",
                             a, x.3, x.2, x.1, x.0, a + x.3 + x.2 + x.1 + x.0)
                })
        }

        primes.push(a);
    }
}

fn quad_groups(list: &Vec<usize>) -> Vec<(usize, usize, usize, usize)> {
    let length = list.len();
    let mut out: Vec<(usize, usize, usize, usize)> = vec![];

    for i in 0..length {
        for j in (i + 1)..length {
            for k in (j + 1)..length {
                for l in (k + 1)..length {
                    out.push((list[i], list[j], list[k], list[l]))
                }
            }
        }
    }

    out
}

fn triple_groups(list: &Vec<usize>) -> Vec<(usize, usize, usize)> {
    let length = list.len();
    let mut out: Vec<(usize, usize, usize)> = vec![];

    for i in 0..length {
        for j in (i + 1)..length {
            for k in (j + 1)..length {
                out.push((list[i], list[j], list[k]))
            }
        }
    }

    out
}

fn quad_directional_concat_prime(a: usize, b: usize, c: usize, d: usize) -> bool {
    tri_directional_concat_prime(a, b, c)
        && bi_directional_concat_prime(a, d)
        && bi_directional_concat_prime(b, d)
        && bi_directional_concat_prime(c, d)
}

fn tri_directional_concat_prime(a: usize, b: usize, c: usize) -> bool {
    bi_directional_concat_prime(a, b)
        && bi_directional_concat_prime(b, c)
        && bi_directional_concat_prime(a, c)
}

fn bi_directional_concat_prime(a: usize, b: usize) -> bool {
    primal::is_prime(concat(a, b)) && primal::is_prime(concat(b, a))
}

fn concat(a: usize, b: usize) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}