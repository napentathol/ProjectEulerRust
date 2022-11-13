
fn main() {
    let mut prime = 3;
    let mut total = 5;
    let mut i = 1;

    while prime * 100 / total >= 10 {
        i += 1;

        let tr = top_right(i);
        let tl = top_left(i);
        let bl = bottom_left(i);

        if primal::is_prime(tr) { prime += 1 }
        if primal::is_prime(tl) { prime += 1 }
        if primal::is_prime(bl) { prime += 1 }

        total += 4;
    }

    println!("{}", side_length(i));
}

fn top_right(i: u64) -> u64 { corner(i, 2) }
fn top_left(i: u64) -> u64 { corner(i, 4) }
fn bottom_left(i: u64) -> u64 { corner(i, 6) }

fn side_length(i: u64) -> u64 { i * 2 + 1 }

fn corner(i: u64, d_init: u64) -> u64 {
    (1..=i).map(|x| d_init + 8 * (x - 1)).sum::<u64>() + 1
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
}