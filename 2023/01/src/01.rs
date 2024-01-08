use aho_corasick::{AhoCorasick, Match};
aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut total: i32 = 0;

    for line in input.lines() {
        let nums: Vec<u8> = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();
        let first = *nums.first().unwrap() as i32;
        let last = *nums.last().unwrap() as i32;
        total += first * 10 + last;
    }
    total
}

fn part_2(input: aoc::Input) -> impl ToString {
    let digits = [
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ];
    let mut total: i32 = 0;

    let ac = AhoCorasick::new(digits).unwrap();
    for line in input.lines() {
        let matches: Vec<Match> = ac.find_overlapping_iter(line).collect();
        let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
        let last = matches.last().unwrap().pattern().as_i32() / 2 + 1;

        total += first * 10 + last;
    }
    total
}
