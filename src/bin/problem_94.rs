use num_integer::Roots;

const N_MAX : u64 = 1_000_000_000;

fn main() {
    let mut perimeter_sum = 0;

    let mut n : u64 = 3;
    while n * 3 + 1 < N_MAX {
        let m = n + 1;
        let half_m = m / 2;
        let h = (n * n - half_m * half_m).sqrt();

        if half_m * half_m + h * h == n * n {
            let area = m * h / 2;

            println!("f({n}) =\t(\t{m},\t{half_m},\t{h},\t{area})");
            perimeter_sum += n*2 + m;
        }

        n += 2;
    }

    let mut n : u64 = 3;
    while n * 3 - 1 < N_MAX {
        let m = n - 1;
        let half_m = m / 2;
        let h = (n * n - half_m * half_m).sqrt();

        if half_m * half_m + h * h == n * n {
            let area = m * h / 2;

            println!("f({n}) =\t(\t{m},\t{half_m},\t{h},\t{area})");
            perimeter_sum += n*2 + m;
        }

        n += 2;
    }
    println!("perimeter sum: {perimeter_sum}")
}