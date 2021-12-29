use std::collections::HashMap;

fn parse(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    let mut res = Vec::new();
    let coords = |s: &str| {
        let (x, y) = s.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };
    for line in input.lines() {
        let (a, b) = line.split_once(" -> ").unwrap();
        res.push((coords(a), coords(b)));
    }
    res
}

pub fn part_a(input: &str) -> i64 {
    let lines = parse(input);
    let mut map: HashMap<(_, _), i64> = HashMap::new();
    let mut overlaps = 0;

    for ((x1, y1), (x2, y2)) in lines {
        if x1 != x2 && y1 != y2 {
            // only considering vertical & horizontal lines
            continue;
        }
        let ((x1, y1), (x2, y2)) = ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2)));
        for x in x1..=x2 {
            for y in y1..=y2 {
                let stored = map.entry((x, y)).or_default();
                *stored += 1;
                if *stored == 2 {
                    overlaps += 1;
                }
            }
        }
    }
    overlaps
}

// creates an endless cycle in the range [a,b]
fn range(a: isize, b: isize) -> impl Iterator<Item = isize> {
    std::iter::successors(Some(a), move |n| Some(n + (b - a).signum()))
}

// Calculate the absolute difference between usize values
fn absdiff(a: usize, b: usize) -> usize {
    let max = a.max(b);
    let min = a.min(b);
    max - min
}

pub fn part_b(input: &str) -> i64 {
    let lines = parse(input);
    let mut map: HashMap<(_, _), i64> = HashMap::new();
    let mut overlaps = 0;

    for ((x1, y1), (x2, y2)) in lines {
        let range_x = range(x1 as isize, x2 as isize);
        let range_y = range(y1 as isize, y2 as isize);
        let length = absdiff(x1, x2).max(absdiff(y1, y2)) + 1;
        for coord in range_x.zip(range_y).take(length) {
            let stored = map.entry(coord).or_default();
            *stored += 1;
            if *stored == 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

pub fn solve() {
    let input = std::fs::read_to_string("day05/input.txt").unwrap();
    println!("Day05:");

    let now = std::time::Instant::now();
    let ans = part_a(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part a: {} (took {} us)", ans, elapsed);

    let now = std::time::Instant::now();
    let ans = part_b(input.as_str());
    let elapsed = now.elapsed().as_micros();
    println!("  part b: {} (took {} us)", ans, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parta() {
        assert_eq!(part_a(EXAMPLE), 5);
    }

    #[test]
    fn test_partb() {
        assert_eq!(part_b(EXAMPLE), 12);
    }
}
