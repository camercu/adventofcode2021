// Parses lines in format <direction> <amount>
pub fn parse(input: &str) -> Vec<(String, i64)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut action = line.split_whitespace();
        let direction = action.next().unwrap().to_lowercase();
        let amount = action.next().unwrap().parse().unwrap();
        result.push((direction, amount));
    }
    result
}

pub fn part_a(input: &str) -> (i64, i64) {
    let mut horizontal = 0;
    let mut depth = 0;

    for (direction, amount) in parse(input) {
        match direction.as_str() {
            "forward" => {
                horizontal += amount;
            }
            "up" => {
                depth -= amount;
            }
            "down" => {
                depth += amount;
            }
            _ => {
                panic!();
            }
        }
    }
    (horizontal, depth)
}

pub fn part_b(input: &str) -> (i64, i64) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (direction, amount) in parse(input) {
        match direction.as_str() {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => {
                aim -= amount;
            }
            "down" => {
                aim += amount;
            }
            _ => {
                panic!();
            }
        }
    }
    (horizontal, depth)
}

pub fn solve() {
    let input = std::fs::read_to_string("day02/input.txt").unwrap();
    println!("Day02:");

    let now = std::time::Instant::now();
    let (horiz, depth) = part_a(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part a: {} (took {} us)", horiz * depth, elapsed);

    let now = std::time::Instant::now();
    let (horiz, depth) = part_b(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part b: {} (took {} us)", horiz * depth, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_parta() {
        assert_eq!(part_a(EXAMPLE), (15, 10));
    }

    #[test]
    fn test_partb() {
        assert_eq!(part_b(EXAMPLE), (15, 60));
    }
}
