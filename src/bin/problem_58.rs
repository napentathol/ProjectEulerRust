use project_euler_rust::prime_sieve::PrimeSieveData;

fn main() {
    let mut prime_sieve = PrimeSieveData::new();

    let mut prime = 3;
    let mut total = 5;
    let mut i = 1;

    while prime * 100 / total > 10 {
        i += 1;

        let tr = top_right(i);
        let tl = top_left(i);
        let bl = bottom_left(i);
        let br = bottom_right(i);

        if prime_sieve.check_is_prime(tr) { prime += 1 }
        if prime_sieve.check_is_prime(tl) { prime += 1 }
        if prime_sieve.check_is_prime(bl) { prime += 1 }

        total += 4;

        println!("side length: {}", side_length(i));
        println!("\ttop_right: {}", tr);
        println!("\ttop_left: {}", tl);
        println!("\tbottom_left: {}", br);
        println!("\tbottom_right: {}", bl);
        println!();
        println!("\tprimes: {}", prime);
        println!("\ttotal: {}", total);
        println!("\tpercentage: {}", prime * 100 / total);
        println!();
    }

    println!("{}", side_length(i));
}

fn top_right(i: usize) -> usize { corner(i, 2) }
fn top_left(i: usize) -> usize { corner(i, 4) }
fn bottom_left(i: usize) -> usize { corner(i, 6) }
fn bottom_right(i: usize) -> usize { corner(i, 8) }

fn side_length(i: usize) -> usize { i * 2 + 1 }

fn corner(i: usize, d_init: usize) -> usize {
    (1..=i).map(|x| d_init + 8 * (x - 1)).sum::<usize>() + 1
}

#[test]
fn test_top_right() {
    assert_eq!(1, top_right(0));
    assert_eq!(3, top_right(1));
    assert_eq!(13, top_right(2));
    assert_eq!(31, top_right(3));

    assert_eq!(1, top_left(0));
    assert_eq!(5, top_left(1));
    assert_eq!(17, top_left(2));
    assert_eq!(37, top_left(3));

    assert_eq!(1, bottom_left(0));
    assert_eq!(7, bottom_left(1));
    assert_eq!(21, bottom_left(2));
    assert_eq!(43, bottom_left(3));

    assert_eq!(1, bottom_right(0));
    assert_eq!(9, bottom_right(1));
    assert_eq!(25, bottom_right(2));
    assert_eq!(49, bottom_right(3));
}