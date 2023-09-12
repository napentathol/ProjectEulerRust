fn main() {
    let mut x = 1;

    while rect_count(x, x) < 2_000_000 {
        x += 1;
    }

    let mut y = x;
    let mut smallest_diff = rect_count(x, y);

    while y > 0 {
        let mut res = 0;

        while res < 2_000_000 {
            res = rect_count(x, y);
            let diff = (res - 2_000_000).abs();
            let area = x * y;

            if diff < smallest_diff {
                println!("{x},\t{y}:\t\t{res}\t\t{diff}\t\t{area}");
                smallest_diff = diff
            }

            x += 1;
        }

        y -= 1;
    }
}

fn rect_count(x: i64, y: i64) -> i64 {
    side_sum(x) * side_sum(y)
}

fn side_sum(n: i64) -> i64 {
    n * (n + 1) / 2
}