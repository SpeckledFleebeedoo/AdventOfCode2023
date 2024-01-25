struct RaceParameters {
    time: u64,
    distance: u64,
}

fn calc_ways_to_win(race: RaceParameters) -> i32 {
    let mut ways_to_win = 0;
    for charge_time in 0..race.time {
        let total_distance = (race.time - charge_time) * charge_time;
        if total_distance > race.distance {
            ways_to_win += 1;
        };
    };
    return ways_to_win;
}

fn part1() {
    let mut races: Vec<RaceParameters> = Vec::new();
    races.push(RaceParameters{time: 50, distance: 242});
    races.push(RaceParameters{time: 74, distance: 1017});
    races.push(RaceParameters{time: 86, distance: 1691});
    races.push(RaceParameters{time: 85, distance: 1252});

    let mut ways_to_win_vec = Vec::new();

    for race in races {
        let ways_to_win = calc_ways_to_win(race);
        ways_to_win_vec.push(ways_to_win);
    }
    println!("Part 1: {}", ways_to_win_vec.iter().fold(1, |res, a| res * a))
}

fn part2() {
    let race = RaceParameters{time: 50748685, distance: 242101716911252};
    let ways_to_win = calc_ways_to_win(race);
    println!("Part 2: {}", ways_to_win)
}

fn main() {
    part1();
    part2();
}