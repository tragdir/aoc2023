aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut result: usize = 0;
    'start: for line in input.lines() {
        let (games, turns) = line.split_once(": ").unwrap();
        let (_, id) = games.split_once(' ').unwrap();
        let id: usize = id.parse().unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();

        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();
            for c in cubes {
                let (amount, color) = c.split_once(' ').unwrap();
                let amount: usize = amount.parse().unwrap();
                match &color[0..1] {
                    "r" => turn.red = amount,
                    "g" => turn.green = amount,
                    "b" => turn.blue = amount,
                    _ => panic!("messed up"),
                }
            }
            if turn.red > 12 || turn.green > 13 || turn.blue > 14 {
                continue 'start;
            }
        }
        result += id;
    }
    result
}

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut power_sum = 0;

    for game in input.lines() {
        let (_, turns) = game.split_once(": ").unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            for c in cubes {
                let (amount, color) = c.split_once(' ').unwrap();
                let amount: usize = amount.parse().unwrap();

                match color {
                    "red" => red = red.max(amount),
                    "green" => green = green.max(amount),
                    "blue" => blue = blue.max(amount),
                    _ => panic!("messed up"),
                }
            }
        }
        power_sum += red * green * blue;
    }
    power_sum
}
