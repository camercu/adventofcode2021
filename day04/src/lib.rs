use std::collections::{BTreeMap, HashSet};

const ROW: usize = 0b11111;
const COL: usize = 0b100001000010000100001;

// Returns 2-tuple (nums, boards):
// nums: vector of numbers called
// boards: vector of boards as map of (number: index) pairs
fn parse(input: &str) -> (Vec<usize>, Vec<BTreeMap<usize, usize>>) {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let nums: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();
    let boards: Vec<BTreeMap<usize, usize>> = boards
        .split("\n\n")
        .map(|board| {
            board
                .split_whitespace()
                .enumerate()
                .map(|(i, n)| (n.parse().unwrap(), i))
                .collect()
        })
        .collect();
    (nums, boards)
}

// return True if a complete row or column of numbers are marked
fn check_win(marks: &usize) -> bool {
    (0..5).any(|i| (marks >> (i * 5) & ROW == ROW) || (marks >> i & COL == COL))
}

// return the sum of numbers that aren't marked on the board
fn sum_unmarked(board: &BTreeMap<usize, usize>, marks: &usize) -> usize {
    board
        .iter()
        .filter(|(_, &idx)| marks & (1 << idx) == 0)
        .map(|(n, _)| n)
        .sum()
}

// Bingo game. Given input of numbers called in order and boards in play.
// Once a winner is found, returns the "score," which is the sum of all
// unmarked numbers on board times the last number called.
pub fn part_a(input: &str) -> usize {
    let (nums, boards) = parse(input);

    // marking called numbers on each board
    let mut marks = vec![0; boards.len()];
    for num in nums {
        for (b, board) in boards.iter().enumerate() {
            match board.get(&num) {
                Some(i) => {
                    marks[b] |= 1 << i;
                    if check_win(&marks[b]) {
                        return num * sum_unmarked(board, &marks[b]);
                    }
                }
                None => continue,
            }
        }
    }

    panic!("No winner found!");
}

pub fn part_b(input: &str) -> usize {
    let (nums, boards) = parse(input);
    let mut in_play: HashSet<_> = (0..boards.len()).collect();

    let mut marks = vec![0; boards.len()];
    for num in nums {
        for (b, board) in boards.iter().enumerate() {
            if !in_play.contains(&b) {
                continue;
            }
            match board.get(&num) {
                Some(i) => {
                    marks[b] |= 1 << i;
                    if check_win(&marks[b]) {
                        in_play.remove(&b);
                        if in_play.is_empty() {
                            return num * sum_unmarked(&boards[b], &marks[b]);
                        }
                    }
                }
                None => continue,
            }
        }
    }

    0
}

pub fn solve() {
    let input = std::fs::read_to_string("day04/input.txt").unwrap();
    println!("Day04:");

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
mod test {
    use super::*;
    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_check_win() {
        assert!(check_win(&0b1111100000));
        assert!(!check_win(&0b11111000)); // streak not row-aligned
        assert!(check_win(&0b1000010000100001000010000));
        assert!(!check_win(&0b1000100000100001000010000)); // not column-aligned
    }

    #[test]
    fn test_sum_unmarked() {
        let (_, boards) = parse(EXAMPLE);
        let board = &boards[0];
        let top_row_sum = 22 + 13 + 17 + 11 + 0;
        let top_row_unmarked = 0b11111_11111_11111_11111_00000;
        assert_eq!(top_row_sum, sum_unmarked(board, &top_row_unmarked));
    }

    #[test]
    fn test_parta() {
        assert_eq!(4512, part_a(EXAMPLE));
    }

    #[test]
    fn test_partb() {
        assert_eq!(1924, part_b(EXAMPLE));
    }
}
