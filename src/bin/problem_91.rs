fn main() {
    print_right_triangle_count(2);
    print_right_triangle_count(50);
}

fn print_right_triangle_count(n: i32) {
    let res = right_triangle_count(n);

    println!("{n}: {res}");
}

fn right_triangle_count(n: i32) -> i32 {
    let mut right_triangle = 0;
    let max = (n + 1) * (n + 1);

    for i in 1..(max - 1) {
        let (x_1, y_1) = convert_index_to_point(n, i);
        for j in (i + 1)..max {
            let (x_2, y_2) = convert_index_to_point(n, j);

            let a = x_1 == 0 && y_2 == 0;
            let b = (x_1 * (x_1 - x_2)) == - (y_1 * (y_1 - y_2));
            let c = (x_2 * (x_1 - x_2)) == - (y_2 * (y_1 - y_2));

            if a || b || c {
                right_triangle += 1;
            }
        }
    }
    return right_triangle
}

fn convert_index_to_point(n: i32, index: i32) -> (i32, i32) { (index / (n + 1), index % (n + 1)) }
