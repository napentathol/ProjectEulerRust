use std::cmp::min;
use num_integer::Roots;

fn main() {
    print_combinations_to_52();
    println!();
    let ttoc = do_calculation();
    println!("{}", ttoc)
}

fn print_combinations_to_52() {
    for i in 1..=52 {
        for j in 0..= min(13,i) {
            for k in 0..= min(6,j) {
                for l in 0..= min(1,k) {
                    if ((2*i + 1) * (2*j + 1) * (2*k + 1) * (2*l + 1) - 1) / 2 == 52 {
                        println!("{}, {}, {}, {}", i, j, k, l)
                    }
                }
            }
        }
    }
}

const TEN_TO_THE_ELEVEN: u64 = 10_u64.pow(11);

fn do_calculation() -> u64 {
    println!("entering ttoc");
    let (non_hypotenuse_numbers, running_sum) =
        prepopulate_non_hypotenuse_numbers_and_sum_upto(TEN_TO_THE_ELEVEN/(5*5*5*13*13*17));
    println!("done precalculating non-hypotenuse primes");

    let mut out: u64 = 0;

    let mut i: u64 = 1;
    // hypotenuses of the form i^3 * j^2 * k
    while i.pow(3) < TEN_TO_THE_ELEVEN / 325 {
        i += 4;
        if !primal::is_prime(i) { continue; }

        let mut j: u64 = 1;
        while j.pow(2) < TEN_TO_THE_ELEVEN / 5 / i.pow(3) {
            j += 4;
            if i == j || !primal::is_prime(j) { continue; }

            let mut k: u64 = 1;

            while k < TEN_TO_THE_ELEVEN / i.pow(3) / j.pow(2) {
                k += 4;
                if i == k || j == k || !primal::is_prime(k) { continue; }

                let val = i.pow(3) * j.pow(2) * k;
                if val < TEN_TO_THE_ELEVEN {
                    let d = TEN_TO_THE_ELEVEN / val;
                    out += running_sum[find_count_before(d, &non_hypotenuse_numbers) as usize - 1] * val
                }
            }
        }
    }
    println!("done with hypotenuses of the form i^3 * j^2 * k");

    let mut i: u64 = 1;
    // hypotenuses of the form i^7 * j^3
    while i.pow(7) < TEN_TO_THE_ELEVEN / 125 {
        i += 4;
        if !primal::is_prime(i) { continue; }

        let mut j: u64 = 1;
        while j.pow(3) < TEN_TO_THE_ELEVEN / i.pow(7) {
            j += 4;
            if i == j || !primal::is_prime(j) { continue; }

            let val = i.pow(7) * j.pow(3);
            if val < TEN_TO_THE_ELEVEN {
                let d = TEN_TO_THE_ELEVEN / val;
                out += running_sum[find_count_before(d, &non_hypotenuse_numbers) as usize - 1] * val
            }
        }
    }
    println!("done with hypotenuses of the form i^7 * j^3");

    let mut i: u64 = 1;
    // hypotenuses of the form i^10 * j^2
    while i.pow(10) < TEN_TO_THE_ELEVEN / 25 {
        i += 4;
        if !primal::is_prime(i) { continue; }

        let mut j: u64 = 1;
        while j.pow(2) < TEN_TO_THE_ELEVEN / i.pow(10) {
            j += 4;
            if i == j || !primal::is_prime(j) { continue; }

            let val = i.pow(10) * j.pow(2);
            if val < TEN_TO_THE_ELEVEN {
                let d = TEN_TO_THE_ELEVEN / val;
                out += running_sum[find_count_before(d, &non_hypotenuse_numbers) as usize - 1] * val
            }
        }
    }
    println!("done with hypotenuses of the form i^10 * j^2");

    return out
}

fn find_count_before(n: u64, v: &Vec<u64>) -> u64 {
    if v.first().map(|x| *x > n).unwrap_or(true) { return 0 }
    if v.last().map(|x| *x <= n).unwrap_or(false) { return v.len() as u64 }

    find_count_before_rec(n, v, 1, v.len())
}

#[test]
fn test_find_count_before() {
    let test_vec = &vec![2,3,5,8,13,21];

    assert_eq!(0, find_count_before(1, test_vec));
    assert_eq!(1, find_count_before(2, test_vec));
    assert_eq!(2, find_count_before(3, test_vec));
    assert_eq!(2, find_count_before(4, test_vec));
    assert_eq!(3, find_count_before(5, test_vec));
    assert_eq!(3, find_count_before(6, test_vec));
    assert_eq!(3, find_count_before(7, test_vec));
    assert_eq!(4, find_count_before(8, test_vec));
    assert_eq!(5, find_count_before(13, test_vec));
    assert_eq!(6, find_count_before(21, test_vec));
    assert_eq!(6, find_count_before(34, test_vec));
}

fn find_count_before_rec(n: u64, v: &Vec<u64>, min: usize, max: usize) -> u64 {
    let pos = ((max - min) / 2) + min;
    let v2 = v[pos];
    if v2 <= n { return find_count_before_rec(n, v, pos, max) }
    let v1 = v[pos - 1];
    if v1 > n { return find_count_before_rec(n, v, min, pos) }

    return pos as u64
}

fn prepopulate_non_hypotenuse_numbers_and_sum_upto(upto: u64) -> (Vec<u64>, Vec<u64>) {
    let non_hypotenuse_numbers: Vec<u64> = (1..=upto)
        .filter(|x| is_non_hypotenuse(*x))
        .collect();

    let mut running_sum: Vec<u64> = vec![non_hypotenuse_numbers[0]];
    for i in 1..non_hypotenuse_numbers.len() {
        let prev = running_sum[i - 1];
        let sum = prev + non_hypotenuse_numbers[i];
        running_sum.push(sum);
    }

    return (non_hypotenuse_numbers, running_sum)
}

#[test]
fn test_prepopulate() {
    let (non_hypotenuse_numbers, running_sum) =
        prepopulate_non_hypotenuse_numbers_and_sum_upto(25);

    assert_eq!(
        vec![1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 14, 16, 18, 19, 21, 22, 23, 24],
        non_hypotenuse_numbers
    );
    assert_eq!(
        vec![1, 3, 6, 10, 16, 23, 31, 40, 51, 63, 77, 93, 111, 130, 151, 173, 196, 220],
        running_sum
    );
}

fn is_non_hypotenuse(x: u64) -> bool {
    if x % 4 == 1 && primal::is_prime(x) { return false }

    let mut n = 2;
    while n <= x.sqrt() {

        if primal::is_prime(n) && n % 4 == 1 && x % n == 0 {
            return false;
        }

        let d = x as f64 / n as f64;
        if d % 4.0 == 1.0 && primal::is_prime(d as u64) && d % 4.0 == 1.0 {
            return false;
        }

        n += 1;
    }
    return true;
}

#[test]
fn test_is_non_hypotenuse() {
    let hyp_n = vec![
        5, 10, 13, 15, 17, 20, 25, 26, 29, 30, 34, 35, 37, 39, 40, 41, 45, 50, 51, 52, 53, 55, 58,
        60, 61, 65, 68, 70, 73, 74, 75, 78, 80, 82, 85, 87, 89, 90, 91, 95, 97, 100, 101, 102, 104,
        105, 106, 109, 110, 111, 113, 115, 116, 117, 119, 120, 122, 123, 125, 130, 135, 136, 137,
        140
    ];
    for n in hyp_n {
        assert_eq!(false, is_non_hypotenuse(n))
    }

    let non_hyp_n = vec![
        1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 14, 16, 18, 19, 21, 22, 23, 24, 27, 28, 31, 32, 33, 36, 38,
        42, 43, 44, 46, 47, 48, 49, 54, 56, 57, 59, 62, 63, 64, 66, 67, 69, 71, 72, 76, 77, 79, 81,
        83, 84, 86, 88, 92, 93, 94, 96, 98, 99, 103, 107, 108, 112, 114, 118, 121, 124, 126, 127
    ];
    for n in non_hyp_n {
        assert_eq!(true, is_non_hypotenuse(n))
    }
}

/*
this didn't work

fn main() {
    let start = Instant::now();
    for i in 1..=32 {
        println!("{}: {}", i, n_points(i));
    }
    let duration = start.elapsed();
    println!("type 1, 32 elements, took: {:?}", duration);

    let start = Instant::now();
    let squares = &squares_up_to(32);
    for i in 1..=32 {
        println!("{}: {}", i, squares_method_n_points(i, squares));
    }
    let duration = start.elapsed();
    println!("type 2, 32 elements, took: {:?}", duration);

    let start = Instant::now();
    for i in 1..=256 {
        println!("{}: {}", i, n_points(i));
    }
    let duration = start.elapsed();
    println!("type 1, 256 elements, took: {:?}", duration);


    let start = Instant::now();
    let squares = &squares_up_to(256);
    for i in 1..=256 {
        println!("{}: {}", i, squares_method_n_points(i, squares));
    }
    let duration = start.elapsed();
    println!("type 2, 256 elements, took: {:?}", duration);

    let start = Instant::now();
    for i in 1..=8192 {
        println!("{}: {}", i, n_points(i));
    }
    let duration = start.elapsed();
    println!("type 1, 8192 elements, took: {:?}", duration);


    let start = Instant::now();
    let squares = &squares_up_to(8192);
    for i in 1..=8192 {
        println!("{}: {}", i, squares_method_n_points(i, squares));
    }
    let duration = start.elapsed();
    println!("type 2, 8192 elements, took: {:?}", duration);

    let start = Instant::now();
    for i in 1..=10_000 {
        println!("{}: {}", i, n_points(i));
    }
    let duration = start.elapsed();
    println!("type 1, 10,000 elements, took: {:?}", duration);

    let start = Instant::now();
    let squares = &squares_up_to(10_000);
    for i in 1..=10_000 {
        println!("{}: {}", i, squares_method_n_points(i, squares));
    }
    let duration = start.elapsed();
    println!("type 2, 10,000 elements, took: {:?}", duration);

    let start = Instant::now();
    let squares = &squares_up_to(10_u64.pow(11));
    let duration = start.elapsed();
    println!("{}: {}", 100_000_000_000_u64, squares_method_n_points(100_000_000_000, squares));
    println!("type 2, 100,000,000,000 elements, took: {:?}", duration);

}

fn n_points(n: u64) -> u64 {
    let half_n: f64 = (n as f64) / 2_f64;
    let radius_squared: f64 = (n as f64) * half_n;

    let mut count = 4;

    for i in 1..=(half_n.floor() as u64) {
        let width = half_n - (i as f64);
        let height = (radius_squared - (width * width)).sqrt();
        let y = height + half_n;
        if y % 1.0 == 0.0 {
            /*
            println!("n({}), half_n({}), radius_squared({}): (x = {}, y = {}) (width = {}, height = {})",
                     n, half_n, radius_squared, i, y, width, height);

            println!("\t4xrsq({}) | 2xnn({}) | dwsq({}) dhsq({}) sum({})", 4.0 * radius_squared,
                     n*n*2,
                     (2.0*height) * (2.0*height), (2.0*width) * (2.0*width),
                     (2.0*height) * (2.0*height) + (2.0*width) * (2.0*width));*/


            count += 8
        }
    }

    return count
}

fn squares_method_n_points(n: u64, squares: &Vec<u64>) -> u64 {
    let tnn = 2 * n * n;

    let mut sum = 4;
    for s in squares {
        if *s > n { break; }
        if (tnn - s).is_square() { sum += 8 }
    }
    sum
}

fn squares_up_to(n: u64) -> Vec<u64> {
    (1..=n).map(|x| x * x).collect()
}

trait IsSquare {
    fn is_square(&self) -> bool;
}

impl IsSquare for u64 {
    fn is_square(&self) -> bool {
        let sqrt = (*self as f64).sqrt().round() as u64;
        *self == sqrt * sqrt
    }
}
*/