fn main() {
    let res = (mod_pow(2, 7830457, 10_000_000_000) * 28433 + 1) % 10_000_000_000;

    println!("{}", res)
}

fn mod_pow(base: u128, exp: u128, n: u128) -> u128 {
    if exp == 0 { return 1 }

    let mut b = base;
    let mut x = exp;
    let mut res = 1;

    b %= n;

    loop {
        if x % 2 == 1 {
            res *= b;
            res %= n;
        }

        if x == 1 {
            return res;
        }

        x /= 2;
        b *= b.clone();
        b %= n;
    }
}