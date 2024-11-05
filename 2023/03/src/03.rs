aoc::parts!(1);

use std::collections::HashSet;

#[derive(Default, Debug)]
#[allow(unused)]
struct DataSet {
    nums: Vec<PartNumber>,
    symbols: HashSet<(i64, i64)>,
}

#[derive(Default, Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left side
            (row - 1, col),
            (row + 1, col), // above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right side
        ]);

        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);

        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
    }
}

fn parse(input: aoc::Input) -> DataSet {
    let mut current: Option<PartNumber> = None;
    let mut result: DataSet = DataSet::default();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = current {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    current = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = current.take() {
                    result.nums.push(num);
                }

                if ch != '.' {
                    result.symbols.insert((row as i64, col as i64));
                }
            }
        }
    }

    result
}

fn part_1(input: aoc::Input) -> impl ToString {
    let data = parse(input);
    let total = data
        .nums
        .iter()
        .filter(|num| num.points.intersection(&data.symbols).next().is_some())
        .map(|num| num.value)
        .sum::<i64>();

    total
}

#[allow(dead_code, unused_variables)]
fn part_2(input: aoc::Input) -> impl ToString {
    0
}
