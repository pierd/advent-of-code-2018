use std::env;

fn power_level(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;
    let hundreds = (rack_id * y + serial_number) * rack_id / 100 % 10;
    hundreds as isize - 5
}

fn main() {
    let mut args = env::args();
    args.next(); // skip program name
    let serial_number: usize = args.next().unwrap().parse().unwrap();

    let mut sums = [[0; 300]; 300];
    for x in 0..300 {
        let mut row_sum = 0;
        for y in 0..300 {
            row_sum += power_level(x + 1, y + 1, serial_number);
            sums[x][y] = row_sum + if x > 0 { sums[x - 1][y] } else { 0isize };
        }
    }

    let mut max_val = -100;
    let mut max_x = 0;
    let mut max_y = 0;
    for x in 0..297 {
        for y in 0..297 {
            let val = sums[x + 3][y + 3] + sums[x][y] - sums[x + 3][y] - sums[x][y + 3];
            if max_val < val {
                max_val = val;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("{},{} = {}", max_x + 2, max_y + 2, max_val);

    let mut max_s = 1;
    for s in 1..300 {
        for x in 0..(300 - s) {
            for y in 0..(300 - s) {
                let val = sums[x + s][y + s] + sums[x][y] - sums[x + s][y] - sums[x][y + s];
                if max_val < val {
                    max_val = val;
                    max_x = x;
                    max_y = y;
                    max_s = s;
                }
            }
        }
    }
    println!("{},{},{} = {}", max_x + 2, max_y + 2, max_s, max_val);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
