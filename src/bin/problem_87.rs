use itertools::Itertools;
use num_integer::Roots;

fn main() {
    generate_prime_power_triples_below(50).iter().for_each(|x| println!("{}", x));

    let below_50k = generate_prime_power_triples_below(50_000);
    println!("below 50k: {}", below_50k.len());

    let below_50m = generate_prime_power_triples_below(50_000_000);
    println!("below 50M: {}", below_50m.len());
}

fn generate_prime_power_triples_below(n: u64) -> Vec<u64> {
    let fourthable_limit = n.nth_root(4);
    let fourthables = primal::Primes::all()
        .take_while(|p| (*p as u64) <= fourthable_limit)
        .map(|x| x as u64);

    return fourthables.flat_map(|i| {
        let i_4 = i*i*i*i;
        let cubable_limit = (n - i_4).nth_root(3);

        let cubables = primal::Primes::all()
            .take_while(move |p| (*p as u64) <= cubable_limit)
            .map(|x| x as u64);

        cubables.flat_map(move |j| {
            let j_3 = j*j*j;
            let square_limit = (n - i_4 - j_3).sqrt();

            primal::Primes::all()
                .take_while(|p| (*p as u64) <= square_limit)
                .map(|x| x as u64)
                .map(|k| k * k + j_3 + i_4)
                .filter(|x| *x <= n)
                .collect::<Vec<u64>>()
        })
    })
        .sorted()
        .dedup()
        .collect()
}

#[test]
fn check_roots_behavior() {
    assert_eq!(84, 50_000_000.nth_root(4));
    assert_eq!(368, 50_000_000.nth_root(3));
    assert_eq!(7071, 50_000_000.sqrt());
}

/*
this didn't work

    let mut s: HashSet<u64> = HashSet::new();

    let mut n: u64 = 1;
    while n < 50_000_000 {
        let mut k: u64 = 1;
        'checkLoop: while n > k*k*k*k {

            let mut j: u64 = 1;
            while n - k*k*k*k > j*j*j {
                let i: u64 = (n - k*k*k*k - j*j*j).sqrt();
                let m = i*i + j*j*j + k*k*k*k;
                if m == n {
                    println!("{} = {}^4 + {}^3 + {}^2", n, k, j, i);
                    s.insert(m);
                    break 'checkLoop;
                }
                j += 1;
            }

            k += 1;
        }

        n += 1;
    }

    println!("{}", s.len())
}
 */