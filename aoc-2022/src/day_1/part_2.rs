use std::collections::BinaryHeap;
use std::io::{self, BufRead, Lines};

pub fn part_2() {
    let mut lines = io::stdin().lines();

    let calories = parse_max_calories(&mut lines);

    assert_eq!(calories.len(), 3);
    println!("Max calories: {}", calories.iter().sum::<u32>());
}

fn parse_max_calories<B: BufRead>(lines: &mut Lines<B>) -> Vec<u32> {
    let mut heap = BinaryHeap::<u32>::new();

    loop {
        let calories = parse_calories(lines);
        heap.push(calories);

        if calories == 0 {
            break; // EOF
        }
    }

    assert!(heap.len() > 2);

    let mut max_calories = Vec::new();

    for _ in 0..3 {
        max_calories.push(heap.pop().unwrap());
    }

    max_calories
}

fn parse_calories<B: BufRead>(lines: &mut Lines<B>) -> u32 {
    let mut calories: u32 = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break; // Empty line between two elves
        }

        calories += line.parse::<u32>().unwrap();
    }

    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_calory() {
        let zero = String::from("0");
        let line = io::IoSlice::new(zero.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 0);
    }

    #[test]
    fn one_thousand_calory() {
        let calory = String::from("1000");
        let line = io::IoSlice::new(calory.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 1000);
    }

    #[test]
    fn one_hundred_calories() {
        let calories = String::from("20\n50\n30");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(parse_calories(&mut line.lines()), 100);
    }

    #[test]
    fn four_elves_carrying_same_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60\n\n10\n\n0");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(
            parse_max_calories(&mut line.lines()).iter().sum::<u32>(),
            210
        );
    }

    #[test]
    fn four_elves_second_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n100\n40\n60\n10\n\n250\n\n20");
        let line = io::IoSlice::new(calories.as_bytes());

        assert_eq!(
            parse_max_calories(&mut line.lines()).iter().sum::<u32>(),
            560
        );
    }
}