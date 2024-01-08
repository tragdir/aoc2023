use aho_corasick::AhoCorasick;
aoc::parts!(1, 2);

#[allow(dead_code)]
const DIGITS: [&str; 18] = [
    "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8",
    "eight", "9", "nine",
];

fn part_1(input: aoc::Input) -> impl ToString {
    let mut total: i32 = 0;

    for line in input.lines() {
        let nums = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();
        let first = *nums.first().unwrap();
        let last = *nums.last().unwrap();
        total += (first as i32) * 10 + last as i32;
    }
    total.to_string()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut total: i32 = 0;

    let ac = AhoCorasick::new(DIGITS).unwrap();
    for line in input.lines() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = matches.first().unwrap().pattern().as_usize() / 2 + 1;
        let last = matches.last().unwrap().pattern().as_usize() / 2 + 1;

        total += (first as i32) * 10 + last as i32;
    }
    total.to_string()
}
