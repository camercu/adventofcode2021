use itertools::Itertools;

// count number of depth increases
pub fn part_a(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.parse().unwrap())
        .tuple_windows::<(i64, i64)>()
        .filter(|(a, b)| b > a)
        .count()
}

// count number of depth increases between 3-measurement groups
pub fn part_b(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.parse().unwrap())
        .tuple_windows::<(i64, _, _, i64)>()
        .filter(|(a, _, _, b)| b > a)
        .count()
}

pub fn solve() {
    let input = std::fs::read_to_string("day01/input.txt").unwrap();
    println!("Day01:");

    let now = std::time::Instant::now();
    let parta = part_a(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part a: {} (took {} us)", parta, elapsed);

    let now = std::time::Instant::now();
    let partb = part_b(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part b: {} (took {} us)", partb, elapsed);
}

#[cfg(test)]
mod tests {

    use crate::*;
    const EXAMPLE: &'static str = "\
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_a() {
        assert_eq!(7, part_a(EXAMPLE));
    }

    #[test]
    fn test_b() {
        assert_eq!(5, part_b(EXAMPLE))
    }
}
