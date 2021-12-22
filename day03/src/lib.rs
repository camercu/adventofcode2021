fn most_common_nth_bit(data: &[&str], n: usize) -> i64 {
    let mut tally = 0;
    for line in data {
        match line.chars().nth(n).unwrap() {
            '0' => tally -= 1,
            '1' => tally += 1,
            _ => panic!(),
        }
    }

    if tally >= 0 {
        1
    } else {
        0
    }
}

pub fn part_a(input: &str) -> (i64, i64) {
    let data: Vec<&str> = input.lines().collect();
    let size = data[0].len();

    // gamma bits set 1 if 1 is most common
    let gamma = (0..size).fold(0, |gamma, i| (gamma << 1) | most_common_nth_bit(&data, i));

    // epsilon bits set 1 if 1 is least common
    let epsilon = !gamma & ((1 << size) - 1);

    (gamma, epsilon)
}

pub fn part_b(input: &str) -> (i64, i64) {
    let mut oxy: Vec<&str> = input.lines().collect();
    let mut co2: Vec<&str> = input.lines().collect();
    let size = oxy[0].len();

    for i in 0..size {
        let oxy_bit = most_common_nth_bit(&oxy, i) as u8 + 0x30;
        let co2_bit = (1 - most_common_nth_bit(&co2, i)) as u8 + 0x30;

        if oxy.len() > 1 {
            oxy.retain(|&line| line.chars().nth(i).unwrap() == oxy_bit as char);
        }
        if co2.len() > 1 {
            co2.retain(|&line| line.chars().nth(i).unwrap() == co2_bit as char);
        }
    }

    let oxy = i64::from_str_radix(oxy[0], 2).unwrap();
    let co2 = i64::from_str_radix(co2[0], 2).unwrap();

    (oxy, co2)
}

pub fn solve() {
    let input = std::fs::read_to_string("day03/input.txt").unwrap();
    println!("Day03:");

    let now = std::time::Instant::now();
    let (gamma, epsilon) = part_a(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part a: {} (took {} us)", gamma * epsilon, elapsed);

    let now = std::time::Instant::now();
    let (oxy, co2) = part_b(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part b: {} (took {} us)", oxy * co2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_most_common_nth_bit() {
        let data: Vec<&str> = EXAMPLE.lines().collect();
        assert_eq!(most_common_nth_bit(&data, 0), 1);
        assert_eq!(most_common_nth_bit(&data, 1), 0);
        assert_eq!(most_common_nth_bit(&data, 2), 1);
        assert_eq!(most_common_nth_bit(&data, 3), 1);
        assert_eq!(most_common_nth_bit(&data, 4), 0);
    }

    #[test]
    fn test_parta() {
        assert_eq!(part_a(EXAMPLE), (22, 9));
    }

    #[test]
    fn test_partb() {
        assert_eq!(part_b(EXAMPLE), (23, 10));
    }
}
