#[derive(PartialEq)]
enum Rock {
    Round,
    Square,
    None,
}

fn main() {
    let file_path = "./input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut grid: Vec<Vec<Rock>> = Vec::new();

    for i in 0..lines.len() {
        grid.push(Vec::new());
        for char in lines[i].trim().chars() {
            grid[i].push(match char {
                'O' => Rock::Round,
                '#' => Rock::Square,
                '.' => Rock::None,
                _ => panic!("Invalid character found"),
            });
        }
    }

    // Move all rocks north
    // Let the great migration begin
    // I guess this is kinda similar to bubblesort... Rocks float, apparently.

    loop {
        let mut rocks_moved = 0;
        for y in 1..grid.len() { // No need to check the top row
            for x in 0..grid[0].len() {
                if grid[y][x] == Rock::Round && grid[y-1][x] == Rock::None {
                    grid[y-1][x] = Rock::Round;
                    grid[y][x] = Rock::None;
                    rocks_moved += 1;
                }
            }
        }
        println!("{}", rocks_moved);
        if rocks_moved == 0 {
            break;
        }
    }

    let mut total_load = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Rock::Round {
                let load = grid.len() - y;
                total_load += load;
            }
        }
    }

    println!("Total load: {}", total_load)

}
